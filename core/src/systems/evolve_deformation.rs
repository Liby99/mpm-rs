use specs::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

fn clamp(n: f32, theta_c: f32, theta_s: f32) -> f32 {
  f32::min(f32::max(n, 1.0 - theta_c), 1.0 + theta_s)
}

fn clamp_sigma(sigma: Vector3f, theta_c: f32, theta_s: f32) -> Vector3f {
  Vector3f::new(
    clamp(sigma.x, theta_c, theta_s),
    clamp(sigma.y, theta_c, theta_s),
    clamp(sigma.z, theta_c, theta_s),
  )
}

pub struct EvolveDeformationSystem;

impl<'a> System<'a> for EvolveDeformationSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    WriteStorage<'a, ParticleDeformation>,
  );

  fn run(&mut self, (dt, grid, positions, mut deformations): Self::SystemData) {
    (&positions, &mut deformations)
      .par_join()
      .for_each(|(position, def)| {
        let mut grad_vp = Matrix3f::zeros();
        for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
          let node = grid.get_node(node_index);
          grad_vp += node.velocity * grad_w.transpose();
        }

        let temp_factor = Matrix3f::identity() + dt.get() * grad_vp;
        let temp_f = temp_factor * (def.f_elastic * def.f_plastic);
        let temp_f_e = temp_factor * def.f_elastic;
        // let temp_f_p = def.f_plastic;

        let svd = temp_f_e.svd(true, true);
        match (svd.u, svd.v_t) {
          (Some(u), Some(v_t)) => {

            let sigma_v = clamp_sigma(svd.singular_values, def.theta_c, def.theta_s);
            let sigma = Matrix3f::from_diagonal(&sigma_v);
            let sigma_inv = Matrix3f::from_diagonal(&Vector3f::repeat(1.0).component_div(&sigma_v));
            let new_f_e = u * sigma * v_t;
            let new_f_p = v_t.transpose() * sigma_inv * u.transpose() * temp_f;

            def.f_elastic = new_f_e;
            def.f_plastic = new_f_p;
          },
          _ => panic!("Cannot decompose svd"),
        }
      })
  }
}
