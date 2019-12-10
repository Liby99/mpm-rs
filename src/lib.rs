extern crate nalgebra as na;

mod math;
mod utils;

pub use math::*;
pub use utils::*;

pub struct Particle {
  pub mass: f32,
  pub position: Vector3f,
  pub velocity: Vector3f,
}

pub struct Cell {
  pub mass: f32,
  pub index: Vector3i,
  pub velocity: Vector3f,
}

impl Cell {
  pub fn clean(&mut self) {
    self.mass = 0.0;
    self.velocity = Vector3f::zeros();
  }
}

pub struct Grid {
  pub cell_h: f32,
  pub dimension: Vector3i,
  cells: Vec<Cell>,
}

impl Grid {
  pub fn clean(&mut self) {
    for cell in &mut self.cells {
      cell.clean();
    }
  }

  fn raw_index(&self, cell_index: Vector3i) -> usize {
    self.dimension.x * self.dimension.y * cell_index.z + self.dimension.x * cell_index.y + cell_index.x
  }

  pub fn get_cell(&self, cell_index: Vector3i) -> &Cell {
    let index = self.raw_index(cell_index);
    &self.cells[index]
  }

  pub fn get_cell_mut(&mut self, cell_index: Vector3i) -> &mut Cell {
    let index = self.raw_index(cell_index);
    &mut self.cells[index]
  }

  pub fn transfer_mass(&mut self, particles: &Vec<Particle>) {
    // TODO
  }

  pub fn transfer_momentum(&mut self, particles: &Vec<Particle>) {
    // TODO
  }

  pub fn set_velocity(&mut self) {
    // TODO
  }

  pub fn apply_gravity(&mut self) {
    // TODO
  }
}

pub struct World {
  pub grid: Grid,
  pub particles: Vec<Particle>,
}

impl World {
  pub fn g2p(&mut self) {
    // TODO: Do nothing
  }

  pub fn move_particles(&mut self) {
    // TODO: Do nothing
  }

  pub fn step(&mut self) {
    // 1. Clean grid data by zeroing out everything.
    self.grid.clean();

    // 2. Transfer mass from particles to grid using
    self.grid.transfer_mass(&self.particles);

    // 3. Transfer momentum to grid using
    self.grid.transfer_momentum(&self.particles);

    // 4. Go over all grid nodes, if mi = 0, set vi = 0. Otherwise, set
    self.grid.set_velocity();

    // 5. Apply gravity on grid
    self.grid.apply_gravity();

    // 6. Interpolate new velocity back to particles
    self.g2p();

    // 7. Move the particles
    self.move_particles();
  }
}
