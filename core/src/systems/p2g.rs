use specs::prelude::*;

use crate::components::*;
use crate::resources::*;

pub struct P2GSystem;

impl<'a> System<'a> for P2GSystem {
  type SystemData = (
    Write<'a, Grid>,
    ReadStorage<'a, ParticleMass>,
    ReadStorage<'a, ParticleVelocity>,
    ReadStorage<'a, ParticlePosition>,
  );

  fn run(&mut self, (mut grid, masses, velocities, positions): Self::SystemData) {
    for (mass, velocity, position) in (&masses, &velocities, &positions).join() {
      for (node_index, weight, _) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node_mut(node_index);
        node.mass += mass.get() * weight;
        node.momentum += mass.get() * velocity.get() * weight;
      }
    }
  }
}