use specs::prelude::*;

use crate::resources::*;

pub struct GridF2VSystem;

impl<'a> System<'a> for GridF2VSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Write<'a, Grid>,
  );

  fn run(&mut self, (dt, mut grid): Self::SystemData) {
    for node in &mut grid.nodes {
      if node.mass != 0.0 {
        node.velocity += node.force / node.mass * dt.get();
      }
    }
  }
}