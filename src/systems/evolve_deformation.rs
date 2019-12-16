use specs::prelude::*;

use crate::utils::*;
use crate::resources::*;
use crate::components::*;

pub struct EvolveDeformationSystem;

impl<'a> System<'a> for EvolveDeformationSystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Read<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    WriteStorage<'a, ParticleDeformation>,
  );

  fn run(&mut self, (dt, grid, positions, mut deformations): Self::SystemData) {
    (&positions, &mut deformations).par_join().for_each(|(position, deformation)| {
      let mut grad_vp = Matrix3f::zeros();
      for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node(node_index);
        grad_vp += node.velocity * grad_w.transpose();
      }
      let new_deformation = (Matrix3f::identity() + dt.get() * grad_vp) * deformation.get();
      deformation.set(new_deformation);
    })
  }
}