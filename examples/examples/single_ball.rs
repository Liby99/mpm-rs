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
  let ball_center = Vector3f::new(0.5, 0.4, 0.5);
  let ball_radius = 0.1;
  let ball_velocity = Vector3f::zeros();
  let ball_mass = 10.0;
  let num_particles = 10000;

  // Create output directory
  std::fs::create_dir_all(outdir).unwrap();

  // Initialize the world, use PlyDumpSystem to output ply to `outdir`
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(PlyDumpSystem::new(outdir, dump_skip))
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

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}
