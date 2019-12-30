use specs::prelude::*;
use rayon::prelude::*;

use crate::resources::Grid;

pub struct CleanGridSystem;

impl<'a> System<'a> for CleanGridSystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      node.clean();
    })
  }
}