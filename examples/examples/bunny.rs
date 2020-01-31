use mpm_rs::*;
use mpm_examples::*;
use msh_rs::*;
use nalgebra as na;

fn main() {
  run_example(
    Config {
      output_directory: "result/bunny",
      num_cycles: 5000,
      dump_skip: 20,
      world_dt: 0.001,
      ..Default::default()
    },
    |world| {

      // Put the boundary
      world.put_friction_boundary(0.04, 1.4);

      // Put the bunny
      let bunny = TetrahedronMesh::load("res/bunny.msh").unwrap();
      let translation = na::Translation3::from(Vector3f::new(0.5, 0.3, 0.5));
      let rotation = na::UnitQuaternion::identity();
      let scale = 3.0;
      let transf = na::Similarity3::from_parts(translation, rotation, scale);
      world
        .put_tetra_mesh(&bunny, na::convert(transf), 20.0)
        .with(ParticleVelocity::new(Vector3f::new(-3.0, 1.0, -8.0)))
        .with(ParticleDeformation::elastic(150000.0, 0.3));

      // Make the world only show a portion
      world.hide_random_portion(0.9);
    }
  )
}
