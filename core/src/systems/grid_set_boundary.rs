use rayon::prelude::*;
use specs::prelude::*;

use crate::resources::*;
use crate::utils::*;

pub struct GridSetBoundarySystem;

impl<'a> System<'a> for GridSetBoundarySystem {
  type SystemData = Write<'a, Grid>;

  fn run(&mut self, mut grid: Self::SystemData) {
    grid.nodes.par_iter_mut().for_each(|node| match node.boundary {
      Boundary::None => {}
      Boundary::Sticky => {
        node.velocity = Vector3f::zeros();
      }
      Boundary::Sliding { normal } | Boundary::Friction { normal, .. } => {
        node.velocity -= f32::min(Vector3f::dot(&node.velocity, &normal), 0.0) * normal;
      }
    })
  }
}
