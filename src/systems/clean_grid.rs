use specs::prelude::*;

use crate::resources::Grid;

pub struct CleanGridSystem;

impl<'a> System<'a> for CleanGridSystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    for node in &mut grid.nodes {
      node.clean();
    }
  }
}