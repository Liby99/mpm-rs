use mpm_rs::*;

fn main() {

  // Before all, create the directory
  let outdir = "single_ball_out".to_string();
  if let Err(err) = std::fs::create_dir(&outdir) {
    println!("Cannot make directory {}: {:?}", outdir, err);
    return;
  }

  // First construct the world
  let grid = Grid::new(0.01, Vector3u::new(100, 100, 100));
  let particles = vec![];
  let mut world = World { grid, particles };

  // Build the boundaries and so on
  put_boundary(&mut world, 0.03);
  put_ball(&mut world, Vector3f::new(0.5, 0.8, 0.5), 0.1, 1000, 1.0);

  // Then build the driver
  let mut driver = Driver::new(world, 0.1);

  // Finally run the driver
  if let Err(err) = driver.run(outdir, 100) {
    println!("IO Error: {:?}", err);
  }
}