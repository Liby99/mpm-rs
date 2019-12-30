use specs::prelude::*;
use rayon::prelude::*;

use crate::resources::*;

pub struct ApplyGravitySystem;

impl<'a> System<'a> for ApplyGravitySystem {
  type SystemData = (
    Read<'a, Gravity>,
    Write<'a, Grid>,
  );

  fn run(&mut self, (gravity, mut grid): Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      node.force += gravity.get() * node.mass;
    })
  }
}