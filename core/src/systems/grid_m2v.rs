use rayon::prelude::*;
use specs::prelude::*;

use crate::resources::*;
use crate::utils::*;

/// Momentum to Velocity System of Grid
pub struct GridM2VSystem;

impl<'a> System<'a> for GridM2VSystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      if node.mass == 0.0 {
        node.velocity_temp = Vector3f::zeros();
      } else {
        node.velocity_temp = node.momentum / node.mass;
      }
    })
  }
}
