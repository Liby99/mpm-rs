use specs::prelude::*;
use rayon::prelude::*;

use crate::resources::*;

pub struct GridSetBoundarySystem;

impl<'a> System<'a> for GridSetBoundarySystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      node.set_boundary_velocity();
    })
  }
}