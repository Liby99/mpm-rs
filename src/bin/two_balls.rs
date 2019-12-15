use pbr::ProgressBar;
use mpm_rs::*;

fn main() {
  let outdir = "result/two_balls_out";
  let cycles = 1500;
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let mu = 10000.0;
  let lambda = 15000.0;
  let dump_skip = 3;
  let boundary_thickness = 0.04;

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
  world.put_boundary(boundary_thickness);
  world.put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.1, Vector3f::zeros(), 10.0, 10000);
  world.put_ball(Vector3f::new(0.54, 0.6, 0.54), 0.05, Vector3f::zeros(), 1.25, 1250);

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}