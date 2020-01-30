extern crate nalgebra as na;
extern crate num_traits;
extern crate rand;
extern crate rand_distr;
#[macro_use]
extern crate lazy_static;

use na::{allocator::*, *};
use num_traits::{Float, NumCast};
use rand::prelude::*;
use rand_distr::*;

fn dim<D: Dim>() -> usize {
  D::try_to_usize().unwrap()
}

pub struct Sampler<N: RealField + Float, D: Dim + DimName>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D> + Allocator<N, D>,
{
  start: VectorN<N, D>,
  size: VectorN<N, D>,
  r: N,
  k: usize,
}

pub type Sampler2f = Sampler<f32, U2>;

pub type Sampler3f = Sampler<f32, U3>;

pub type Sampler2d = Sampler<f64, U2>;

pub type Sampler3d = Sampler<f64, U3>;

impl<N: RealField + Float, D: Dim + DimName> Sampler<N, D>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D> + Allocator<N, D>,
{
  /// Create a new sampler with default values:
  /// Filling a space of [0, 1)^D where D is the dimension
  /// The starting point is (0.5)^D
  /// And the radius of poisson disk 0.05
  pub fn new() -> Self {
    Self {
      start: VectorN::<N, D>::repeat(N::from_f32(0.5).unwrap()),
      size: VectorN::<N, D>::repeat(N::from_f32(1.0).unwrap()),
      r: N::from_f32(0.05).unwrap(),
      k: 30,
    }
  }

  /// Let the sampler fill the space with a given minimum gap `r` between each
  /// pair of neighbors
  pub fn with_radius(mut self, r: N) -> Self {
    self.r = r;
    self
  }

  /// Let the sampler try `k` times for each iteration
  pub fn with_k(mut self, k: usize) -> Self {
    self.k = k;
    self
  }

  /// Give the sampler a given starting point
  pub fn with_start(mut self, start: VectorN<N, D>) -> Self {
    self.start = start;
    self
  }

  /// Give the sampler a new random starting point
  pub fn with_random_start(self) -> Self {
    let mut rng = rand::thread_rng();
    let start = self.size.map(|l| {
      let high: f64 = NumCast::from(l).unwrap();
      N::from_f64(rng.gen_range(0.0, high)).unwrap()
    });
    self.with_start(start)
  }

  /// Return the sampler with a given size; will give it a fresh random
  /// starting point within this new size too.
  pub fn with_size(mut self, size: VectorN<N, D>) -> Self {
    self.size = size;
    self.with_random_start()
  }

  /// Generate the samples iterator
  ///
  /// Usage:
  ///
  /// ``` rust
  /// # use nalgebra::U2;
  /// # use poisson::Sampler;
  /// # let sampler = Sampler::<f32, U2>::new();
  /// for sample in sampler.generate() {
  ///   // `sample` is a Vector3f
  ///   // ... do things to sample
  /// }
  /// ```
  pub fn generate(self) -> SamplerIterator<N, D> {
    SamplerIterator::new(self.size, self.r, self.k).with_start(self.start)
  }
}

struct NeighborIterator<D: Dim + DimName>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D>,
{
  dim: VectorN<i64, D>,
  start: VectorN<i64, D>,
  current: usize,
}

impl<D: Dim + DimName> NeighborIterator<D>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D>,
{
  fn new(dim: &VectorN<usize, D>, start: &VectorN<usize, D>) -> Self {
    Self {
      dim: dim.map(|x| x as i64),
      start: start.map(|x| x as i64),
      current: 0,
    }
  }
}

impl<D: Dim + DimName> Iterator for NeighborIterator<D>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D>,
{
  type Item = VectorN<usize, D>;

  fn next(&mut self) -> Option<Self::Item> {
    lazy_static! {
      static ref CHOICES: [i64; 5] = [-2, -1, 0, 1, 2];
    }
    let dim = dim::<D>();
    let len = CHOICES.len();
    loop {
      if self.current >= len.pow(dim as u32) {
        return None;
      } else {
        let mut div = self.current;
        self.current += 1;

        // Construct the local index
        let mut curr_idx = self.start.clone();
        for n in 0..dim {
          let rem = div % len;
          div /= len;
          let choice = CHOICES[rem as usize].clone();
          curr_idx[n] += choice;
        }

        // Check if curr_idx is valid
        let valid = (0..dim).all(|i| 0 <= curr_idx[i] && curr_idx[i] < self.dim[i]);
        if valid {
          return Some(curr_idx.map(|x| x as usize));
        }
      }
    }
  }
}

pub struct SamplerIterator<N: RealField + Float, D: Dim + DimName>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D> + Allocator<N, D>,
{
  grid_cells: Vec<Option<VectorN<N, D>>>,
  grid_dx: N,
  grid_dim: VectorN<usize, D>,
  size: VectorN<N, D>,
  active_samples: Vec<VectorN<N, D>>,
  k: usize,
  r: N,
  rng: ThreadRng,
}

impl<N: RealField + Float, D: Dim + DimName> SamplerIterator<N, D>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D> + Allocator<N, D>,
{
  fn new(size: VectorN<N, D>, r: N, k: usize) -> Self {
    lazy_static! {
      static ref SQRT_2: f64 = 2.0f64.sqrt();
    }

    // First compute dx and dim
    let grid_dx = r / N::from_f64(*SQRT_2).unwrap();
    let grid_dim: VectorN<usize, D> = size.map(|l| NumCast::from(l / grid_dx).unwrap());

    // Generate grid cells
    let mut num_cells = 1;
    for res in grid_dim.iter() {
      num_cells *= res;
    }
    let grid_cells = vec![None; num_cells];

    // Active
    let active_samples = vec![];

    // Random generator
    let rng = rand::thread_rng();

    Self {
      grid_cells,
      grid_dx,
      grid_dim,
      size,
      active_samples,
      k,
      r,
      rng,
    }
  }

  fn with_start(mut self, start: VectorN<N, D>) -> Self {
    // First put this into active samples
    self.active_samples.push(start.clone());

    // Put this into grid cells
    let idx = self.cell_idx(&start).unwrap();
    self.grid_cells[idx] = Some(start);

    // Return self
    self
  }

  fn is_in_range(&self, v: &VectorN<N, D>) -> bool {
    (0..dim::<D>())
      .map(|n| (v[n], self.size[n]))
      .all(|(x, s)| N::from(0).unwrap() <= x && x < s)
  }

  fn cell_idx(&self, v: &VectorN<N, D>) -> Option<usize> {
    self.cell_hd_idx(v).map(|idx| self.cell_idx_from_hd_idx(&idx))
  }

  fn cell_hd_idx(&self, v: &VectorN<N, D>) -> Option<VectorN<usize, D>> {
    let idx: VectorN<i64, D> = v.map(|x| NumCast::from(x / self.grid_dx).unwrap());
    if (0..dim::<D>()).all(|i| 0 <= idx[i] && idx[i] < (self.grid_dim[i] as i64)) {
      Some(idx.map(|i| i as usize))
    } else {
      None
    }
  }

  fn cell_idx_from_hd_idx(&self, v: &VectorN<usize, D>) -> usize {
    let dim = dim::<D>();
    let mut idx = v[0];
    for i in 1..dim {
      idx = idx * self.grid_dim[i] + v[i];
    }
    idx
  }

  fn random_offset_on_annulus(&mut self) -> VectorN<N, D> {
    let r_f64: f64 = NumCast::from(self.r).unwrap();
    loop {
      let result = VectorN::<N, D>::zeros().map(|_| {
        let random: f64 = self.rng.sample(StandardNormal);
        NumCast::from(random).unwrap()
      });
      let norm = result.normalize();
      let random_len: f64 = self.rng.gen_range(0.0, r_f64 * 2.0);
      let point = norm * N::from_f64(random_len).unwrap();
      if point.magnitude() >= self.r {
        return point;
      }
    }
  }

  fn neighbor_indices(&self, index: &VectorN<usize, D>) -> NeighborIterator<D> {
    NeighborIterator::new(&self.grid_dim.clone(), index)
  }

  fn is_disk_free(&self, point: &VectorN<N, D>) -> bool {
    if let Some(hd_idx) = self.cell_hd_idx(point) {
      let sq_radius = Float::powi(self.r, 2);
      self.neighbor_indices(&hd_idx).all(|hd_idx| {
        let idx = self.cell_idx_from_hd_idx(&hd_idx);
        if let Some(other_point) = &self.grid_cells[idx] {
          let diff = other_point - point;
          let sq_mag = diff.magnitude_squared();
          sq_radius < sq_mag
        } else {
          true
        }
      })
    } else {
      false
    }
  }

  fn insert_if_valid(&mut self, point: &VectorN<N, D>) -> bool {
    if self.is_disk_free(point) {
      if let Some(idx) = self.cell_idx(point) {
        self.active_samples.push(point.clone());
        self.grid_cells[idx] = Some(point.clone());
        true
      } else {
        false
      }
    } else {
      false
    }
  }
}

impl<N: RealField + Float, D: Dim + DimName> Iterator for SamplerIterator<N, D>
where
  DefaultAllocator: Allocator<usize, D> + Allocator<i64, D> + Allocator<N, D>,
{
  type Item = VectorN<N, D>;

  fn next(&mut self) -> Option<Self::Item> {
    while !self.active_samples.is_empty() {
      let active_idx: usize = self.rng.gen_range(0, self.active_samples.len());
      let ref_point = self.active_samples[active_idx].clone();
      for _ in 0..self.k {
        let offset = self.random_offset_on_annulus();
        let point = ref_point.clone() + offset;
        if self.is_in_range(&point) && self.insert_if_valid(&point) {
          return Some(point.clone());
        }
      }
      self.active_samples.swap_remove(active_idx);
    }
    None
  }
}
