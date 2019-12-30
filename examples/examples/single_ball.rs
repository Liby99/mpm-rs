use pbr::ProgressBar;
use mpm_rs::*;

fn main() {
  let cycles = 1500;
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.15;
  let boundary_thickness = 0.04;

  // Initialize the world
  let mut world = WorldBuilder::new(world_size, grid_h).build();

  // Set parameters
  world.set_dt(dt);

  // Put the particles
  world.put_boundary(boundary_thickness);
  world.put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.1, Vector3f::zeros(), 10.0, 10000, youngs_modulus, nu);

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}