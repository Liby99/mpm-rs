use std::marker::PhantomData;
use rand::prelude::*;
use rand_distr::*;

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoissonType {
  Normal,
  Perioditic,
}

fn encode(v: &Vector3f, side: usize, poisson_type: PoissonType) -> Option<usize> {
  let mut index = 0;
  for i in 0..3 {
    let n = v[i];
    let cur = match poisson_type {
      PoissonType::Perioditic => ((n as i64) % side as i64) as usize,
      PoissonType::Normal => {
        if n < 0.0 || n >= side as f32 { return None }
        n as usize
      }
    };
    index = (index + cur) * side;
  }
  Some(index / side)
}

fn decode(index: usize, side: usize) -> Option<Vector3f> {
  let dim = 3;
  if index >= side.pow(dim as u32) {
    return None;
  }
  let mut result = Vector3f::zeros();
  let mut last = index;
  for n in (0..3).rev() {
    let cur = last / side;
    result[n] = (last - cur * side) as f32;
    last = cur;
  }
  Some(result)
}

pub struct PoissonGrid {
  data: Vec<Vec<Vector3f>>,
  side: usize,
  cell: f32,
  poisson_type: PoissonType,
  marker: PhantomData<f32>,
}

impl PoissonGrid {
  pub fn new(radius: f32, ty: PoissonType) -> Self {
    let dim : f32 = 3.0;
    let cell = (2.0 * radius) / dim.sqrt();
    let side = (1.0 / cell) as usize;
    Self {
      cell: cell,
      side: side,
      data: vec![vec![]; side.pow(dim as u32)],
      poisson_type: ty,
      marker: PhantomData,
    }
  }

  pub fn get(&self, index: Vector3f) -> Option<&Vec<Vector3f>> {
    encode(&index, self.side, self.poisson_type).map(|t| &self.data[t])
  }

  pub fn get_mut(&mut self, index: Vector3f) -> Option<&mut Vec<Vector3f>> {
    encode(&index, self.side, self.poisson_type).map(move |t| &mut self.data[t])
  }

  pub fn cells(&self) -> usize {
    self.data.len()
  }
}

pub struct CombiIter<'a> {
  cur: usize,
  choices: &'a [i32],
  _marker: PhantomData<(f32, Vector3f)>,
}

impl<'a> Iterator for CombiIter<'a> {
  type Item = Vector3f;

  fn next(&mut self) -> Option<Self::Item> {
    let dim = 3;
    let len = self.choices.len();
    if self.cur >= len.pow(dim as u32) {
      None
    } else {
      let mut result = Vector3f::zeros();
      let mut div = self.cur;
      self.cur += 1;
      for n in 0..3 {
        let rem = div % len;
        div /= len;
        result[n] = self.choices[rem as usize].clone() as f32;
      }
      Some(result)
    }
  }
}

/// Iterates through all combinations of vectors with allowed values as scalars.
fn each_combination(choices: &[i32]) -> CombiIter {
  CombiIter {
    cur: 0,
    choices: choices,
    _marker: PhantomData,
  }
}

pub struct PoissonDisk {
  radius: f32,
  poisson_type: PoissonType,
  grid: PoissonGrid,
  active_samples: Vec<Vector3f>,
  outside: Vec<Vector3f>,
  success: usize,
  rng: ThreadRng,
}

impl PoissonDisk {
  pub fn new(radius: f32, ty: PoissonType) -> Self {
    Self {
      radius: radius,
      poisson_type: ty,
      grid: PoissonGrid::new(radius, ty),
      active_samples: vec![],
      outside: vec![],
      success: 0,
      rng: rand::thread_rng(),
    }
  }

  fn random_point_annulus(&mut self, min: f32, max: f32) -> Vector3f {
    loop {
      let mut result = Vector3f::zeros();
      for n in 0..3 {
        result[n] = self.rng.sample(StandardNormal);
      }
      let result : Vector3f = result.normalize() * self.rng.gen::<f32>() * max;
      if result.magnitude() >= min {
        return result;
      }
    }
  }

  fn sample_to_index(&self, value: &Vector3f) -> Vector3f {
    let mut cur = value.clone();
    for n in 0..3 {
      cur[n] = (cur[n] * self.grid.side as f32).floor();
    }
    cur
  }

  fn insert_if_valid(&mut self, index: Vector3f, sample: Vector3f) -> bool {
    if self.is_disk_free(index.clone(), 0, sample.clone(), &self.outside) {
      self.active_samples.push(sample.clone());
      self.grid
          .get_mut(index)
          .expect("Because the sample is [0, 1) indexing it should work.")
          .push(sample);
      self.success += 1;
      true
    } else {
      false
    }
  }

  fn sqdist(&self, v1: Vector3f, v2: Vector3f) -> f32 {
    let diff = v2 - v1;
    match self.poisson_type {
      PoissonType::Perioditic => {
        each_combination(&[-1, 0, 1])
          .map(|v| (diff.clone() + v).norm_squared())
          .fold(std::f32::MAX, |a, b| a.min(b))
      },
      PoissonType::Normal => diff.norm_squared(),
    }
  }

  fn is_valid(&self, samples: &[Vector3f], sample: Vector3f) -> bool {
    let sqradius = (2.0 * self.radius).powi(2);
    samples.iter().all(|t| self.sqdist(t.clone(), sample.clone()) >= sqradius)
  }

  fn is_disk_free(&self, index: Vector3f, level: usize, sample: Vector3f, outside: &[Vector3f]) -> bool {
    let parent = self.get_parent(index, level);
    let sqradius = (2.0 * self.radius).powi(2);
    // NOTE: This does unnessary checks for corners, but it doesn't affect much in higher dimensions: 5^d vs 5^d - 2d
    each_combination(&[-2, -1, 0, 1, 2])
      .filter_map(|t| self.grid.get(parent.clone() + t))
      .flat_map(|t| t)
      .all(|v|
        self.sqdist(v.clone(), sample.clone()) >= sqradius)
      && self.is_valid(outside, sample)
  }

  fn get_parent(&self, mut index: Vector3f, level: usize) -> Vector3f {
    let split = 2usize.pow(level as u32);
    for n in 0..3 {
      index[n] = (index[n] / split as f32).floor();
    }
    index
  }

  fn choose_random_sample(&mut self, index: Vector3f, level: usize) -> Vector3f {
    let side = 2usize.pow(level as u32);
    let spacing = self.grid.cell / side as f32;
    let rdv = Vector3f::new(self.rng.gen(), self.rng.gen(), self.rng.gen());
    (index + rdv) * spacing
  }
}

impl Iterator for PoissonDisk {
  type Item = Vector3f;

  fn next(&mut self) -> Option<Self::Item> {
    while !self.active_samples.is_empty() {
      let index = self.rng.sample(Uniform::new(0, self.active_samples.len()));
      let cur = self.active_samples[index].clone();
      for _ in 0..30 {
        let min = 2.0 * self.radius;
        let max = 4.0 * self.radius;
        let sample = cur.clone() + self.random_point_annulus(min, max);
        if (0..3).map(|n| sample[n]).all(|c| 0.0 <= c && c < 1.0) {
          let index = self.sample_to_index(&sample);
          if self.insert_if_valid(index, sample.clone()) {
            return Some(sample);
          }
        }
      }
      self.active_samples.swap_remove(index);
    }
    while self.success == 0 {
      let cell = self.rng.sample(Uniform::new(0, self.grid.cells()));
      let index = decode(cell, self.grid.side).expect(
          "Because we are decoding random index within grid \
          this should work.",
      );
      let sample = self.choose_random_sample(index.clone(), 0);
      if self.insert_if_valid(index, sample.clone()) {
        return Some(sample);
      }
    }
    None
  }
}
