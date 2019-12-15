use pbr::ProgressBar;
use mpm_rs::*;

fn main() {
  let outdir = "result/flying_balls_out";
  let cycles = 2000;
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 100000.0;
  let nu = 0.2;
  let mu = youngs_modulus / (2.0 * (1.0 + nu));
  let lambda = youngs_modulus * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
  let dump_skip = 4;
  let boundary_thickness = 0.04;

  // Log the parameters
  println!("Mu: {}, Lambda: {}", mu, lambda);

  // Create output directory
  std::fs::create_dir_all(outdir).unwrap();

  // Initialize the world
  let mut world = World::new(world_size, grid_h);

  // Set parameters
  world.set_dt(dt);
  world.set_output_dir(outdir);
  world.set_dump_skip(dump_skip);
  world.set_mu(mu);
  world.set_lambda(lambda);

  // Put the particles
  world.put_vel_dim_boundary(boundary_thickness, 0.95);
  world.put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.05, Vector3f::new(5.0, -2.0, 1.0), 1.25, 1250);
  world.put_ball(Vector3f::new(0.58, 0.6, 0.58), 0.1, Vector3f::new(3.0, 5.0, -3.0), 10.0, 10000);
  world.put_ball(Vector3f::new(0.42, 0.6, 0.42), 0.05, Vector3f::new(-4.3, 1.5, 8.0), 1.25, 1250);

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}