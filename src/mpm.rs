use std::fs::File;
use std::io::Write;

use super::math::*;

#[derive(Copy, Clone, Debug)]
pub struct Particle {
  pub mass: f32,
  pub position: Vector3f,
  pub velocity: Vector3f,
  pub force: Matrix3f,
}

impl Particle {
  pub fn new(mass: f32, position: Vector3f) -> Self {
    let velocity = Vector3f::zeros();
    let force = Matrix3f::zeros();
    Self { mass, position, velocity, force }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Boundary {
  SetZero,
  Surface { normal: Vector3f },
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
  pub mass: f32,
  pub index: Vector3u,
  pub velocity: Vector3f,
  pub momentum: Vector3f,
  pub force: Vector3f,
  pub boundary: Option<Boundary>,
}

impl Node {
  pub fn new(index: Vector3u) -> Self {
    Self {
      index,
      mass: 0.0,
      velocity: Vector3f::zeros(),
      momentum: Vector3f::zeros(),
      force: Vector3f::zeros(),
      boundary: None,
    }
  }

  pub fn clean(&mut self) {
    self.mass = 0.0;
    self.velocity = Vector3f::zeros();
    self.momentum = Vector3f::zeros();
    self.force = Vector3f::zeros();
  }

  pub fn set_boundary_velocities(&mut self) {
    if let Some(b) = self.boundary {
      match b {
        Boundary::SetZero => {
          self.velocity = Vector3f::zeros();
        },
        Boundary::Surface { normal } => {
          self.velocity -= Vector3f::dot(&self.velocity, &normal) * normal;
        }
      }
    }
  }
}

pub struct WeightIterator {
  pub h: f32,
  pub dim: Vector3u,
  pub base_node: Vector3i,
  pub curr_node: Vector3i,
  pub wx: Vector3f,
  pub wy: Vector3f,
  pub wz: Vector3f,
  pub dwx: Vector3f,
  pub dwy: Vector3f,
  pub dwz: Vector3f,
}

impl Iterator for WeightIterator {

  /// (Node Index, Weight, Weight Gradient)
  type Item = (Vector3i, f32, Vector3f);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let i = self.curr_node.x as usize;
      let j = self.curr_node.y as usize;
      let k = self.curr_node.z as usize;
      if i < 3 && j < 3 && k < 3 {

        // Get Node
        let node_index = self.base_node + self.curr_node;

        // Calculate weight
        let wi = self.wx[i];
        let wj = self.wy[j];
        let wk = self.wz[k];
        let weight = wi * wj * wk;

        // Calculate weight gradient
        let dwijkdxi = 0.0; // d (w_{ijk}) / d x_i; TODO
        let dwijkdxj = 0.0; // d (w_{ijk}) / d x_j; TODO
        let dwijkdxk = 0.0; // d (w_{ijk}) / d x_k; TODO
        let grad_w = Vector3f::new(dwijkdxi, dwijkdxj, dwijkdxk);

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

        // Check if node is inside the grid
        let x_in = 0 <= node_index.x && node_index.x < self.dim.x as i32;
        let y_in = 0 <= node_index.y && node_index.y < self.dim.y as i32;
        let z_in = 0 <= node_index.z && node_index.z < self.dim.z as i32;
        if x_in && y_in && z_in {
          return Some((node_index, weight, grad_w))
        }

        // If not, then the loop will continue
      } else {
        return None
      }
    }
  }
}

#[derive(Debug)]
pub struct Grid {
  pub h: f32,
  pub dim: Vector3u,
  pub nodes: Vec<Node>,
}

impl Grid {
  pub fn new(dim: Vector3u, h: f32) -> Self {
    let num_nodes = dim.x * dim.y * dim.z;
    let mut nodes = Vec::with_capacity(num_nodes);
    for z in 0..dim.z {
      for y in 0..dim.y {
        for x in 0..dim.x {
          let index = Vector3u::new(x, y, z);
          nodes.push(Node::new(index));
        }
      }
    }
    Self { h, dim, nodes }
  }

  pub fn clean(&mut self) {
    for node in &mut self.nodes {
      node.clean();
    }
  }

  pub fn set_boundary_velocities(&mut self) {
    for node in &mut self.nodes {
      node.set_boundary_velocities();
    }
  }

  fn raw_index(&self, node_index: Vector3i) -> usize {
    let z_comp = self.dim.x * self.dim.y * node_index.z as usize;
    let y_comp = self.dim.x * node_index.y as usize;
    let x_comp = node_index.x as usize;
    z_comp + y_comp + x_comp
  }

  fn get_node(&self, node_index: Vector3i) -> &Node {
    let index = self.raw_index(node_index);
    &self.nodes[index]
  }

  fn get_node_mut(&mut self, cell_index: Vector3i) -> &mut Node {
    let index = self.raw_index(cell_index);
    &mut self.nodes[index]
  }

  fn get_weight_1d(&self, pos: f32) -> (i32, Vector3f, Vector3f) {
    let x = pos / self.h;
    let base_node = (x - 0.5).floor();

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

    (base_node as i32, w, dw)
  }

  pub fn iterate_neighbors(&self, pos: Vector3f) -> WeightIterator {
    let (bnx, wx, dwx) = self.get_weight_1d(pos.x);
    let (bny, wy, dwy) = self.get_weight_1d(pos.y);
    let (bnz, wz, dwz) = self.get_weight_1d(pos.z);
    let h = self.h;
    let dim = self.dim;
    let base_node = Vector3i::new(bnx, bny, bnz);
    let curr_node = Vector3i::zeros();
    WeightIterator {
      h, dim, base_node, curr_node,
      wx, wy, wz, dwx, dwy, dwz
    }
  }
}

#[derive(Debug)]
pub struct World {
  pub grid: Grid,
  pub particles: Vec<Particle>,
}

impl World {
  pub fn new(size: Vector3f, h: f32) -> Self {
    let x_dim = (size.x / h) as usize;
    let y_dim = (size.y / h) as usize;
    let z_dim = (size.z / h) as usize;
    let grid = Grid::new(Vector3u::new(x_dim, y_dim, z_dim), h);
    let particles = vec![];
    Self { grid, particles }
  }

  fn clean_grid(&mut self) {
    self.grid.clean();
  }

  fn p2g(&mut self) {
    for par in &self.particles {
      for (node_index, weight, _) in self.grid.iterate_neighbors(par.position) {
        let node = self.grid.get_node_mut(node_index);
        node.mass += par.mass * weight;
        node.momentum += par.mass * par.velocity * weight;
      }
    }
  }

  fn grid_momentum_to_velocity(&mut self) {
    for node in &mut self.grid.nodes {
      // Check node mass. If 0, then directly set the velocity of node to zero
      if node.mass == 0.0 {
        node.velocity = Vector3f::zeros();
      } else {
        node.velocity = node.momentum / node.mass;
      }
    }
  }

  fn apply_gravity(&mut self) {
    let g = Vector3f::new(0.0, -9.8, 0.0);
    for node in &mut self.grid.nodes {
      node.force += g * node.mass;
    }
  }

  fn apply_elastic_force(&mut self) {
    // TODO
  }

  fn grid_force_to_velocity(&mut self, dt: f32) {
    for node in &mut self.grid.nodes {
      if node.mass != 0.0 {
        node.velocity += node.force / node.mass * dt;
      }
    }
  }

  fn set_boundary_velocities(&mut self) {
    self.grid.set_boundary_velocities();
  }

  fn evolve_particle_force(&mut self, dt: f32) {
    for par in &mut self.particles {
      let this_fp = par.force;
      let mut grad_vp = Matrix3f::zeros();
      for (node_index, _, grad_w) in self.grid.iterate_neighbors(par.position) {
        let node = self.grid.get_node(node_index);
        grad_vp += node.velocity * grad_w.transpose();
      }
      par.force = (Matrix3f::identity() + dt * grad_vp) * this_fp;
    }
  }

  fn g2p(&mut self, dt: f32) {
    for par in &mut self.particles {

      // First clear the velocity
      par.velocity = Vector3f::zeros();

      // Then accumulate velocities from neighbor nodes
      for (node_index, weight, _) in self.grid.iterate_neighbors(par.position) {
        let node = self.grid.get_node(node_index);
        par.velocity += node.velocity * weight;
      }

      // Finally move the particles using the velocity
      par.position += dt * par.velocity;
    }
  }

  pub fn step(&mut self, dt: f32) {

    // 1. Clean grid data by zeroing out everything.
    self.clean_grid();

    // 2. Transfer particles to grid, will give mass and momentum to grid nodes
    self.p2g();

    // 3. Go over all grid nodes, convert momentum to velocity
    self.grid_momentum_to_velocity();

    // 4. Apply forces on grid
    self.apply_gravity();
    self.apply_elastic_force();

    // 4.1. Turn force into velocity
    self.grid_force_to_velocity(dt);

    // 5. Clean the boundary
    self.set_boundary_velocities();

    // 6. Evolve particle force
    self.evolve_particle_force(dt);

    // 7. Interpolate new velocity back to particles, and move the particles
    self.g2p(dt);
  }

  pub fn dump(&self, filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(b"POINTS\n")?;
    for (i, par) in self.particles.iter().enumerate() {
      let pos = par.position;
      let line = format!("{}: {} {} {}\n", i + 1, pos.x, pos.y, pos.z);
      file.write(line.as_bytes())?;
    }
    file.write(b"POLYS\nEND\n")?;
    Ok(())
  }
}
