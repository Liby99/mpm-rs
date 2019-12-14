use specs::prelude::*;

use crate::utils::*;
use crate::resources::*;
use crate::components::*;

pub struct G2PSystem;

impl<'a> System<'a> for G2PSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, Grid>,
    WriteStorage<'a, ParticleVelocity>,
    WriteStorage<'a, ParticlePosition>,
  );

  fn run(&mut self, (dt, grid, mut velocities, mut positions): Self::SystemData) {
    for (velocity, position) in (&mut velocities, &mut positions).join() {

      // First calculate the new velocity of particle
      let mut new_vel = Vector3f::zeros();
      for (node_index, weight, _) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node(node_index);
        new_vel += weight * node.velocity;
      }

      // Then use forward computation to get new position
      let new_pos = position.get() + new_vel * dt.get();

      // Set the velocity and position
      velocity.set(new_vel);
      position.set(new_pos);
    }
  }
}