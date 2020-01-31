use mpm_rs::*;
use mpm_viewer::*;
use mpm_examples::*;
use msh_rs::*;
use nalgebra as na;

fn main() {
  run_example(
    Config {
      output_directory: "result/bunny_x2",
      num_cycles: 5000,
      dump_skip: 20,
      world_dt: 0.001,
      ..Default::default()
    },
    |world| {
      // Put the boundary
      world.put_friction_boundary(0.04, 1.4);

      // Load the bunny
      let bunny = TetrahedronMesh::load("res/bunny.msh").unwrap();
      let youngs_modulus = 200000.0;
      let nu = 0.2;

      // Put the bunny #1
      let translation_1 = na::Translation3::from(Vector3f::new(0.5, 0.02, 0.5));
      let rotation_1 = na::UnitQuaternion::identity();
      let scale_1 = 2.5;
      let transf_1 = na::Similarity3::from_parts(translation_1, rotation_1, scale_1);
      let color_1 = Color::new(1.0, 0.0, 0.0);
      let mass_1 = 1.25;
      world
        .put_tetra_mesh(&bunny, na::convert(transf_1), mass_1)
        .with(ParticleColor::new(color_1))
        .with(ParticleDeformation::elastic(youngs_modulus, nu));

      // Put the bunny #2
      let translation_2 = na::Translation3::from(Vector3f::new(0.6, 0.33, 0.6));
      let rotation_2 = na::UnitQuaternion::identity();
      let scale_2 = 1.5;
      let transf_2 = na::Similarity3::from_parts(translation_2, rotation_2, scale_2);
      let color_2 = Color::new(0.0, 1.0, 0.0);
      let mass_2 = 0.09;
      world
        .put_tetra_mesh(&bunny, na::convert(transf_2), mass_2)
        .with(ParticleColor::new(color_2))
        .with(ParticleDeformation::elastic(youngs_modulus, nu));
    }
  )
}
