use rayon::prelude::*;
use specs::prelude::*;

use crate::resources::*;
use crate::utils::*; // Grid

pub struct ApplyFrictionSystem;

impl<'a> System<'a> for ApplyFrictionSystem {
  type SystemData = (Read<'a, DeltaTime>, Write<'a, Grid>);

  fn run(&mut self, (dt, mut grid): Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| match node.boundary {
      Boundary::Friction { normal, mu } => {
        let norm_vel = Vector3f::dot(&normal, &node.velocity_temp) * normal;
        let tan_vel = node.velocity_temp - norm_vel;

        let normal_force_mag = -f32::min(Vector3f::dot(&normal, &node.force), 0.0);
        let max_force_mag = node.mass * tan_vel.magnitude() / dt.get();
        let fric_force_mag = f32::min(normal_force_mag * mu, max_force_mag);

        let fric_force_dir = -tan_vel.normalize();

        node.force += fric_force_dir * fric_force_mag;
      }
      _ => {}
    })
  }
}
