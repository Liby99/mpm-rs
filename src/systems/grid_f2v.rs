use specs::prelude::*;
use rayon::prelude::*;

use crate::resources::*;

pub struct GridF2VSystem;

impl<'a> System<'a> for GridF2VSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Write<'a, Grid>,
  );

  fn run(&mut self, (dt, mut grid): Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      if node.mass != 0.0 {
        node.velocity += node.force / node.mass * dt.get();
      }
    })
  }
}