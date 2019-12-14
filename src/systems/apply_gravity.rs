use specs::prelude::*;

use crate::resources::*;

pub struct ApplyGravitySystem;

impl<'a> System<'a> for ApplyGravitySystem {
  type SystemData = (
    Read<'a, Gravity>,
    Write<'a, Grid>,
  );

  fn run(&mut self, (gravity, mut grid): Self::SystemData) {
    for node in &mut grid.nodes {
      node.force += gravity.get() * node.mass;
    }
  }
}