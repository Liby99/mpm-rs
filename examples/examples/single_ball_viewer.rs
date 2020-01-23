use mpm_rs::*;
use mpm_viewer::*;

fn main() {
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.15;
  let boundary_thickness = 0.04;
  let center = Vector3f::new(0.5, 0.7, 0.5);
  let radius = 0.1;
  let mass = 10.0;
  let num_particles = 10000;

  // Initialize the world, use WindowSystem to visualize
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(WindowSystem::new())
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the particles
  world.put_sticky_boundary(boundary_thickness);
  world
    .put_ball(center, radius, mass, num_particles)
    .with(ParticleDeformation::elastic(youngs_modulus, nu));

  // Check the ending state determined by window system.
  // continue if not ended
  while world.not_ending() {
    world.step();
  }
}
