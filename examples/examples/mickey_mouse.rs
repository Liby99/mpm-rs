use mpm_ply_dump::*;
use mpm_rs::*;
use pbr::ProgressBar;

fn main() {
  let outdir = "result/mickey_mouse";
  let cycles = 1500;
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.2;
  let dump_skip = 5;
  let boundary_thickness = 0.04;

  // Create output directory
  std::fs::create_dir_all(outdir).unwrap();

  // Initialize the world
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(PlyDumpSystem::new(outdir, dump_skip))
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the particles
  world.put_sliding_boundary(boundary_thickness);

  // The big ball of mickey mouse
  world
    .put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.1, 10.0, 10000)
    .with(ParticleDeformation::new(youngs_modulus, nu));

  // Left ear
  world
    .put_ball(Vector3f::new(0.58, 0.6, 0.58), 0.05, 1.25, 1250)
    .with(ParticleDeformation::new(youngs_modulus, nu));

  // Right ear
  world
    .put_ball(Vector3f::new(0.42, 0.6, 0.42), 0.05, 1.25, 1250)
    .with(ParticleDeformation::new(youngs_modulus, nu));

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}
