use rayon::prelude::*;
use specs::prelude::*;

use crate::utils::*;
use crate::resources::*; // Grid

pub struct ApplyFrictionSystem;

impl<'a> System<'a> for ApplyFrictionSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Write<'a, Grid>,
  );

  fn run(&mut self, (dt, mut grid): Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| {
      match node.boundary {
        Boundary::Friction { normal, mu } => {

          let normal_force_mag = Vector3f::dot(&normal, &node.force);
          let max_force_mag = node.mass * node.velocity.magnitude() / dt.get();
          let fric_force_mag = f32::min(normal_force_mag * mu, max_force_mag);

          let norm_vel = Vector3f::dot(&normal, &node.velocity) * normal;
          let tan_vel = node.velocity - norm_vel;
          let fric_force_dir = -tan_vel.normalize();

          node.force += fric_force_dir * fric_force_mag;
        },
        _ => {}
      }
    })
  }
}