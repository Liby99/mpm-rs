use specs::prelude::*;
use rayon::prelude::*;

use crate::utils::*;
use crate::resources::*;

/// Momentum to Velocity System of Grid
pub struct GridM2VSystem;

impl<'a> System<'a> for GridM2VSystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      if node.mass == 0.0 {
        node.velocity = Vector3f::zeros();
      } else {
        node.velocity = node.momentum / node.mass;
      }
    })
  }
}