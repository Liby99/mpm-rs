use mpm_rs::*;

fn main() {

  // Before all, create the directory
  let outdir = "result/single_ball_out".to_string();
  if let Err(err) = std::fs::create_dir(&outdir) { panic!(err); }

  // First construct the world
  let mut world = World::new(0.01, Vector3u::new(100, 100, 100));

  // Build the boundaries and so on
  put_boundary(&mut world, 0.03);
  put_ball(&mut world, Vector3f::new(0.5, 0.8, 0.5), 0.05, 1000, 1.0);

  // Then build the driver
  let mut driver = Driver::new(world, 0.01);

  // Finally run the driver
  if let Err(err) = driver.run(outdir, 500) { panic!(err); }
}