use specs::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub struct G2PSystem;

impl<'a> System<'a> for G2PSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, Grid>,
    WriteStorage<'a, ParticleVelocity>,
    WriteStorage<'a, ParticlePosition>,
  );

  fn run(&mut self, (dt, grid, mut velocities, mut positions): Self::SystemData) {
    (&mut velocities, &mut positions)
      .par_join()
      .for_each(|(velocity, position)| {
        // Initialize velocities
        let mut vpic = Vector3f::zeros();
        let mut vflip = velocity.get();

          // First calculate the new velocity of particle
        for (node_index, weight, _) in grid.neighbor_weights(position.get()) {
          let node = grid.get_node(node_index);
          vpic += weight * node.velocity;
          vflip += weight * (node.velocity - node.velocity_temp);
        }

          // Then use forward computation to get new position
        let new_vel = 0.05 * vpic + 0.95 * vflip;
        let new_pos = position.get() + vpic * dt.get();

          // Set the velocity and position
        velocity.set(new_vel);
        position.set(new_pos);
      })
  }
}
