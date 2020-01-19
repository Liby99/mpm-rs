use nalgebra as na;
use mpm_ply_dump::PlyDumpSystem;
use mpm_rs::*;
use msh_rs::*;
use pbr::ProgressBar;
use std::time::SystemTime;

fn main() {
  let start = SystemTime::now();

  // Parameters
  let bunny_file = "res/bunny.msh";
  let outdir = "result/bunny";
  let cycles = 5000;
  let dump_skip = 20;
  let dt = 0.0005;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 150000.0;
  let nu = 0.3;
  let boundary_thickness = 0.04;
  let boundary_fric_mu = 1.4;
  let density = 1500.0;
  let particle_mass = 0.005;
  let bunny_velocity = Vector3f::new(-3.0, 1.0, -8.0);
  let translation = na::Translation3::from(Vector3f::new(0.5, 0.3, 0.5));
  let rotation = na::UnitQuaternion::identity();
  let scale = 3.5;
  let transf = na::Similarity3::from_parts(translation, rotation, scale);
  let output_random_portion = 0.1;

  // Create output directory
  std::fs::create_dir_all(outdir).unwrap();

  // Initialize the world
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(PlyDumpSystem::new(outdir, dump_skip))
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the boundary
  world.put_friction_boundary(boundary_thickness, boundary_fric_mu);

  // Put the bunny
  let bunny = TetrahedronMesh::load(bunny_file).unwrap();
  world
    .put_tetra_mesh(&bunny, na::convert(transf), density, particle_mass)
    .with(ParticleVelocity(bunny_velocity))
    .with(ParticleDeformation::new(youngs_modulus, nu));

  // Make the world only show a portion
  world.only_show_random_portion(output_random_portion);

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }

  // Print finish
  let secs_elapsed = start.elapsed().unwrap().as_secs();
  let finish = format!("Finished {} cycles in {} secs", cycles, secs_elapsed);
  pb.finish_print(finish.as_str());
}
