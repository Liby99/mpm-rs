use mpm_ply_dump::PlyDumpSystem;
use mpm_rs::*;
use msh_rs::{Node, TetrahedronMesh};
use pbr::ProgressBar;
use std::time::SystemTime;

fn node_to_vec(node: &Node) -> Vector3f {
  Vector3f::new(node.x as f32, node.y as f32, node.z as f32)
}

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
  let bunny_scale = 3.5;
  let bunny_offset = Vector3f::new(0.5, 0.3, 0.5);
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
  for tetra in bunny.elems {
    let p1 = node_to_vec(&bunny.nodes[tetra.i1]) * bunny_scale + bunny_offset;
    let p2 = node_to_vec(&bunny.nodes[tetra.i2]) * bunny_scale + bunny_offset;
    let p3 = node_to_vec(&bunny.nodes[tetra.i3]) * bunny_scale + bunny_offset;
    let p4 = node_to_vec(&bunny.nodes[tetra.i4]) * bunny_scale + bunny_offset;
    let a = p2 - p1;
    let b = p3 - p1;
    let c = p4 - p1;
    let volume = Vector3f::dot(&a, &Vector3f::cross(&b, &c)) / 6.0;
    let mass = volume * density;
    let num_pars = mass / particle_mass;
    let par_volume = volume / num_pars;
    for _ in 0..num_pars as usize {
      let pos = random_point_in_tetra(p1, p2, p3, p4);
      world
        .put_particle(pos, particle_mass)
        .with(ParticleVolume(par_volume))
        .with(ParticleVelocity(bunny_velocity))
        .with(ParticleDeformation::new(youngs_modulus, nu));
    }
  }

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
