use mpm_ply_dump::*;
use mpm_rs::*;
use pbr::ProgressBar;

struct Ball {
  center: Vector3f,
  radius: f32,
  mass: f32,
  num_particles: usize,
}

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

  let balls = vec![
    // Head
    Ball {
      center: Vector3f::new(0.5, 0.4, 0.5),
      radius: 0.1,
      mass: 10.0,
      num_particles: 10000,
    },
    // Left ear
    Ball {
      center: Vector3f::new(0.58, 0.6, 0.58),
      radius: 0.05,
      mass: 1.25,
      num_particles: 1250,
    },
    // Right ear
    Ball {
      center: Vector3f::new(0.58, 0.6, 0.58),
      radius: 0.05,
      mass: 1.25,
      num_particles: 1250,
    },
  ];

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

  // Put the balls
  for b in balls {
    world
      .put_ball(b.center, b.radius, b.mass, b.num_particles)
      .with(ParticleDeformation::elastic(youngs_modulus, nu));
  }

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}
