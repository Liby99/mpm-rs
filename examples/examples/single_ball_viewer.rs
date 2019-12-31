use mpm_rs::*;
use mpm_viewer::*;

fn main() {
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 3.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.15;
  let boundary_thickness = 0.04;
  let ball_center = Vector3f::new(0.5, 2.4, 0.5);
  let ball_radius = 0.1;
  let ball_velocity = Vector3f::zeros();
  let ball_mass = 10.0;
  let num_particles = 10000;

  // Initialize the world, use WindowSystem to visualize
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(WindowSystem::new())
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the particles
  world.put_boundary(boundary_thickness);
  world.put_ball(
    ball_center,
    ball_radius,
    ball_velocity,
    ball_mass,
    num_particles,
    youngs_modulus,
    nu,
  );

  // Check the ending state determined by window system.
  // continue if not ended
  while !world.world.fetch::<Ending>().is_ended() {
    world.step();
  }
}
