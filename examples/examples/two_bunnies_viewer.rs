use mpm_rs::*;
use mpm_viewer::*;
use msh_rs::*;
use nalgebra as na;

fn main() {
  let bunny_file = "res/bunny.msh";
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 150000.0;
  let nu = 0.3;
  let boundary_thickness = 0.04;
  let boundary_fric_mu = 1.4;
  let density = 1500.0;
  let particle_mass = 0.001;

  // Transf #1
  let translation_1 = na::Translation3::from(Vector3f::new(0.5, 0.1, 0.5));
  let rotation_1 = na::UnitQuaternion::identity();
  let scale_1 = 2.5;
  let transf_1 = na::Similarity3::from_parts(translation_1, rotation_1, scale_1);
  let color_1 = Color::new(1.0, 0.0, 0.0);

  // Transf #2
  let translation_2 = na::Translation3::from(Vector3f::new(0.6, 0.4, 0.6));
  let rotation_2 = na::UnitQuaternion::identity();
  let scale_2 = 1.5;
  let transf_2 = na::Similarity3::from_parts(translation_2, rotation_2, scale_2);
  let color_2 = Color::new(0.0, 1.0, 0.0);

  // Initialize the world
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(WindowSystem::new())
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the boundary
  world.put_friction_boundary(boundary_thickness, boundary_fric_mu);

  // Load the bunny
  let bunny = TetrahedronMesh::load(bunny_file).unwrap();

  // Put the bunnies; bunny #1 and bunny #2 only differs in transf
  world
    .put_tetra_mesh(&bunny, na::convert(transf_1), density, particle_mass)
    .with(ParticleColor(color_1))
    .with(ParticleDeformation::elastic(youngs_modulus, nu));

  world
    .put_tetra_mesh(&bunny, na::convert(transf_2), density, particle_mass)
    .with(ParticleColor(color_2))
    .with(ParticleDeformation::elastic(youngs_modulus, nu));

  // Print finish
  while world.not_ending() {
    world.step();
  }
}
