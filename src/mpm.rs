use std::fs::File;
use std::io::Write;

use super::math::*;

#[derive(Copy, Clone, Debug)]
pub struct Particle {
  pub mass: f32,
  pub position: Vector3f,
  pub velocity: Vector3f,
}

impl Particle {
  pub fn new(mass: f32, position: Vector3f) -> Self {
    Self { mass, position, velocity: Vector3f::zeros() }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Boundary {
  // pub friction: f32,
  pub normal: Vector3f,
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
      self.velocity -= Vector3f::dot(&self.velocity, &b.normal) * b.normal;
    }
  }
}

pub struct WeightIterator {
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
  type Item = (Vector3i, f32);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let x_in = self.curr_node.x < 3;
      let y_in = self.curr_node.y < 3;
      let z_in = self.curr_node.z < 3;
      if x_in && y_in && z_in {

        // Get Node
        let node_index = self.base_node + self.curr_node;

        // Calculate weight
        let wi = self.wx[self.curr_node.x as usize];
        let wj = self.wy[self.curr_node.y as usize];
        let wk = self.wz[self.curr_node.z as usize];
        let weight = wi * wj * wk;

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
          return Some((node_index, weight))
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
  pub fn new(h: f32, dim: Vector3u) -> Self {
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
    let dim = self.dim;
    let base_node = Vector3i::new(bnx, bny, bnz);
    let curr_node = Vector3i::zeros();
    WeightIterator {
      dim, base_node, curr_node,
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
  pub fn new(h: f32, dim: Vector3u) -> Self {
    let grid = Grid::new(h, dim);
    let particles = vec![];
    Self { grid, particles }
  }

  pub fn clean_grid(&mut self) {
    self.grid.clean();
  }

  pub fn p2g(&mut self) {
    for par in &self.particles {
      for (node_index, weight) in self.grid.iterate_neighbors(par.position) {
        let node = self.grid.get_node_mut(node_index);
        node.mass += par.mass * weight;
        node.momentum += par.mass * par.velocity * weight;
      }
    }
  }

  pub fn momentum_to_velocity(&mut self) {
    for node in &mut self.grid.nodes {

      // Check node mass. If 0, then directly set the velocity of node to zero
      if node.mass == 0.0 {
        node.velocity = Vector3f::zeros();
      } else {
        node.velocity = node.momentum / node.mass;
      }
    }
  }

  pub fn apply_gravity(&mut self) {
    let g = Vector3f::new(0.0, -9.8, 0.0);
    for node in &mut self.grid.nodes {
      node.force += g * node.mass;
    }
  }

  pub fn apply_elastic_force(&mut self) {
    // TODO
  }

  pub fn force_to_velocity(&mut self, dt: f32) {
    for node in &mut self.grid.nodes {
      if node.mass != 0.0 {
        node.velocity += node.force / node.mass * dt;
      }
    }
  }

  fn set_boundary_velocities(&mut self) {
    self.grid.set_boundary_velocities();
  }

  fn g2p(&mut self) {
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

  fn move_particles(&mut self, dt: f32) {
    for par in &mut self.particles {
      par.position += dt * par.velocity;
    }
  }

  pub fn step(&mut self, dt: f32) {

    // 1. Clean grid data by zeroing out everything.
    self.clean_grid();

    // 2. Transfer particles to grid
    self.p2g();

    // 3. Go over all grid nodes, convert momentum to velocity
    self.momentum_to_velocity();

    // 4. Apply forces on grid
    self.apply_gravity();
    self.apply_elastic_force();

    // 4.1. Turn force into velocity
    self.force_to_velocity(dt);

    // 5. Clean the boundary
    self.set_boundary_velocities();

    // 6. Interpolate new velocity back to particles
    self.g2p();

    // 7. Move the particles
    self.move_particles(dt);
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
