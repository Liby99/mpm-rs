use mpm_rs::*;

fn main() {

  // Before all, create the directory
  let outdir = "result/two_balls_out".to_string();
  if let Err(err) = std::fs::create_dir_all(&outdir) { panic!(err); }

  // First construct the world
  let mut world = World::new(Vector3f::new(1.0, 1.0, 1.0), 0.02);

  // Build the boundaries and so on
  world.put_zero_boundary(0.03);
  world.put_ball(Vector3f::new(0.45, 0.25, 0.45), 0.1, 10000, 1.0);
  world.put_ball(Vector3f::new(0.50, 0.45, 0.50), 0.05, 1000, 1.0);

  // Then build the driver
  let mut driver = Driver::new(world, 0.001);

  // Finally run the driver
  if let Err(err) = driver.run(outdir, 500) { panic!(err); }
}