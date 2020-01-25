use rayon::prelude::*;
use specs::prelude::*;

use crate::resources::Grid;
use crate::utils::*;

pub struct CleanGridSystem;

impl<'a> System<'a> for CleanGridSystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      node.mass = 0.0;
      node.velocity_temp = Vector3f::zeros();
      node.velocity = Vector3f::zeros();
      node.momentum = Vector3f::zeros();
      node.force = Vector3f::zeros();
    })
  }
}
