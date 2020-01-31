use specs::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub struct EvolveDeformationSystem;

impl<'a> System<'a> for EvolveDeformationSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    WriteStorage<'a, ParticleDeformation>,
  );

  fn run(&mut self, (dt, grid, positions, mut deformations): Self::SystemData) {
    (&positions, &mut deformations).par_join().for_each(|(position, def)| {
      // First compute gradient v_p
      let mut grad_vp = Matrix3f::zeros();
      for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node(node_index);
        grad_vp += node.velocity * grad_w.transpose();
      }

      // Then compute $\hat{F_{E_p}^{n + 1}}$ and $F_p^{n + 1}$
      let temp_f_e = (Matrix3f::identity() + dt.get() * grad_vp) * def.f_elastic;
      let new_f = temp_f_e * def.f_plastic;

      // Do SVD on temp_f_e
      let svd = temp_f_e.svd(true, true);
      match (svd.u, svd.v_t) {
        (Some(u), Some(v_t)) => {
          // Clamp out values in sigma
          let sigma_hat = svd.singular_values;
          let sigma = Math::clamp_vec(&sigma_hat, 1.0 - def.theta_c, 1.0 + def.theta_s);
          let sigma_inv = Vector3f::new(1.0 / sigma.x, 1.0 / sigma.y, 1.0 / sigma.z);

          // New $F_{E_p}$
          let new_f_e = u * Matrix3f::from_diagonal(&sigma) * v_t;
          let new_f_p = v_t.transpose() * Matrix3f::from_diagonal(&sigma_inv) * u.transpose() * new_f;

          def.f_elastic = new_f_e;
          def.f_plastic = new_f_p;
        }
        _ => panic!("Cannot decompose svd"),
      }
    })
  }
}
