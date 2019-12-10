extern crate nalgebra as na;

mod math;
mod utils;

pub use math::*;
pub use utils::*;

#[derive(Copy, Clone)]
pub struct Particle {
  pub mass: f32,
  pub position: Vector3f,
  pub velocity: Vector3f,
}

#[derive(Copy, Clone)]
pub struct Node {
  pub mass: f32,
  pub index: Vector3i,
  pub velocity: Vector3f,
  pub momentum: Vector3f,
}

impl Node {
  pub fn clean(&mut self) {
    self.mass = 0.0;
    self.velocity = Vector3f::zeros();
    self.momentum = Vector3f::zeros();
  }
}

pub struct Grid {
  pub h: f32,
  pub dimension: Vector3i,
  nodes: Vec<Node>,
}

pub struct WeightIterator {
  pub base_node: Vector3i,
  pub curr_node: Vector3i,
  pub wx: Vector3f,
  pub dwx: Vector3f,
  pub wy: Vector3f,
  pub dwy: Vector3f,
  pub wz: Vector3f,
  pub dwz: Vector3f,
}

impl Iterator for WeightIterator {
  type Item = (Vector3i, f32);

  fn next(&mut self) -> Option<Self::Item> {
    let x_in = self.curr_node.x < 3;
    let y_in = self.curr_node.y < 3;
    let z_in = self.curr_node.z < 3;
    if x_in && y_in && z_in {

      let report_node = self.base_node + self.curr_node;
      let weight = self.wx[self.curr_node.x] * self.wy[self.curr_node.y] * self.wz[self.curr_node.z];

      // Compute the `curr_node` for next step
      if self.curr_node.x == 2 {
        if self.curr_node.y == 2 {
          self.curr_node.z += 1;
          self.curr_node.x = 0;
          self.curr_node.y = 0;
        } else {
          self.curr_node.y += 1;
          self.curr_node.x = 0;
        }
      } else {
        self.curr_node.x += 1;
      }

      Some((report_node, weight))
    } else {
      None
    }
  }
}

impl Grid {
  pub fn clean(&mut self) {
    for node in &mut self.nodes {
      node.clean();
    }
  }

  fn raw_index(&self, node_index: Vector3i) -> usize {
    self.dimension.x * self.dimension.y * node_index.z + self.dimension.x * node_index.y + node_index.x
  }

  pub fn get_node(&self, node_index: Vector3i) -> &Node {
    let index = self.raw_index(node_index);
    &self.nodes[index]
  }

  pub fn get_node_mut(&mut self, cell_index: Vector3i) -> &mut Node {
    let index = self.raw_index(cell_index);
    &mut self.nodes[index]
  }

  pub fn get_base_node_index(&self, par: &Particle) -> Vector3i {
    let diff = Vector3f::new(-0.5, -0.5, -0.5);
    let bp = (par.position + diff) / self.h;
    Vector3i::new(
        bp.x.floor() as usize,
        bp.y.floor() as usize,
        bp.z.floor() as usize,
    )
  }

  pub fn get_weight_1d(&self, pos: f32) -> (usize, Vector3f, Vector3f) {
    let x = (pos - 0.5) / self.h;
    let base_node = x.floor();

    let mut w = Vector3f::zeros();
    let mut dw = Vector3f::zeros();

    let d0 = x - base_node + 1.0;
    let z = 1.5 - d0;
    let z2 = z * z;
    w.x = 0.5 * z2;

    let d1 = d0 - 1.0;
    w.y = 0.75 - d1 * d1;

    let d2 = 1.0 - d1;
    let zz = 1.5 - d2;
    let zz2 = zz * zz;
    w.z = 0.5 * zz2;

    dw.x = -z;
    dw.y = -2.0 * d1;
    dw.z = zz;

    (base_node as usize, w, dw)
  }

  pub fn iterate_neighbors(&self, pos: Vector3f) -> WeightIterator {
    let (bnx, wx, dwx) = self.get_weight_1d(pos.x);
    let (bny, wy, dwy) = self.get_weight_1d(pos.y);
    let (bnz, wz, dwz) = self.get_weight_1d(pos.z);
    let base_node = Vector3i::new(bnx, bny, bnz);
    let curr_node = Vector3i::zeros();
    WeightIterator { base_node, curr_node, wx, wy, wz, dwx, dwy, dwz }
  }

  pub fn transfer_mass(&mut self, particles: &Vec<Particle>) {
    for par in particles {
      for (node_index, weight) in self.iterate_neighbors(par.position) {
        let node = self.get_node_mut(node_index);
        node.mass += par.mass * weight;
      }
    }
  }

  pub fn transfer_momentum(&mut self, particles: &Vec<Particle>) {
    for par in particles {
      for (node_index, weight) in self.iterate_neighbors(par.position) {
        let node = self.get_node_mut(node_index);
        node.momentum += par.mass * par.velocity * weight;
      }
    }
  }

  pub fn set_velocity(&mut self) {
    for node in &mut self.nodes {

      // Check node mass. If 0, then directly set the velocity of node to zero
      if node.mass == 0.0 {
        node.velocity = Vector3f::zeros();
      } else {
        node.velocity = node.momentum / node.mass;
      }
    }
  }

  pub fn apply_gravity(&mut self, dt: f32) {
    let g = Vector3f::new(0.0, -9.8, 0.0);
    for node in &mut self.nodes {
      node.velocity += g * dt;
    }
  }
}

pub struct World {
  pub grid: Grid,
  pub particles: Vec<Particle>,
}

impl World {
  pub fn g2p(&mut self) {
    for par in &mut self.particles {

      // First clear the velocity
      par.velocity = Vector3f::zeros();

      // Then accumulate velocities from neighbor nodes
      for (node_index, weight) in self.grid.iterate_neighbors(par.position) {
        let node = self.grid.get_node(node_index);
        par.velocity += node.velocity * weight;
      }
    }
  }

  pub fn move_particles(&mut self, dt: f32) {
    for par in &mut self.particles {
      par.position += dt * par.velocity;
    }
  }

  pub fn step(&mut self, dt: f32) {
    // 1. Clean grid data by zeroing out everything.
    self.grid.clean();

    // 2. Transfer mass from particles to grid using
    self.grid.transfer_mass(&self.particles);

    // 3. Transfer momentum to grid using
    self.grid.transfer_momentum(&self.particles);

    // 4. Go over all grid nodes, if mi = 0, set vi = 0. Otherwise, set
    self.grid.set_velocity();

    // 5. Apply gravity on grid
    self.grid.apply_gravity(dt);

    // 6. Interpolate new velocity back to particles
    self.g2p();

    // 7. Move the particles
    self.move_particles(dt);
  }
}
