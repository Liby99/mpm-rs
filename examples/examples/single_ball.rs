extern crate nalgebra as na;

use mpm_ply_dump::*;
use mpm_rs::*;
use pbr::ProgressBar;

fn main() {
  let outdir = "result/single_ball";
  let dump_skip = 5;
  let cycles = 1500;
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.15;
  let boundary_thickness = 0.04;
  let center = Vector3f::new(0.5, 0.4, 0.5);
  let radius = 0.1;
  let mass = 10.0;

  // Create output directory
  std::fs::create_dir_all(outdir).unwrap();

  // Initialize the world, use PlyDumpSystem to output ply to `outdir`
  let mut world = WorldBuilder::new()
    .with_size(world_size)
    .with_dx(grid_h)
    .with_dt(dt)
    .with_system(PlyDumpSystem::new(outdir, dump_skip))
    .build();

  // Put the boundary
  world.put_sticky_boundary(boundary_thickness);

  // Put the ball
  {
    let ball = Sphere::new(radius);
    let transf = Translation3f::from(center);
    world
      .put_region(ball, na::convert(transf), mass)
      .with(ParticleDeformation::elastic(youngs_modulus, nu));
  }

  println!("Num Particles: {}", world.num_particles());

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}
