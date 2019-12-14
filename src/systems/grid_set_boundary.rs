use specs::prelude::*;

use crate::resources::*;

pub struct GridSetBoundarySystem;

impl<'a> System<'a> for GridSetBoundarySystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    for node in &mut grid.nodes {
      node.set_boundary_velocity();
    }
  }
}