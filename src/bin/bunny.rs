use pbr::ProgressBar;
use mpm_rs::{World, Vector3f, random_point_in_tetra};
use msh_rs::{TetMesh, Node};

fn node_to_vec(node: &Node) -> Vector3f {
  Vector3f::new(node.x as f32, node.y as f32, node.z as f32)
}

fn main() {
  let bunny_file = "res/bunny.msh";
  let outdir = "result/bunny_out";
  let cycles = 5000;
  let dump_skip = 20;
  let dt = 0.0005;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 150000.0;
  let nu = 0.2;
  let mu = youngs_modulus / (2.0 * (1.0 + nu));
  let lambda = youngs_modulus * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
  let boundary_thickness = 0.04;
  let boundary_velocity_diminishing = 0.95;
  let density = 2500.0;
  let particle_mass = 0.0005;
  let initial_velocity = Vector3f::new(-3.0, 1.0, -8.0);
  let scale = 3.0;
  let offset = Vector3f::new(0.5, 0.3, 0.5);
  let random_portion = 0.25;

  // Log the parameters
  println!("Mu: {}, Lambda: {}", mu, lambda);

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

  // Put the boundary
  world.put_vel_dim_boundary(boundary_thickness, boundary_velocity_diminishing);

  // Put the bunny
  let bunny = TetMesh::load(bunny_file).unwrap();
  for tetra in bunny.tetras {
    let p1 = node_to_vec(&bunny.nodes[tetra.i1]) * scale + offset;
    let p2 = node_to_vec(&bunny.nodes[tetra.i2]) * scale + offset;
    let p3 = node_to_vec(&bunny.nodes[tetra.i3]) * scale + offset;
    let p4 = node_to_vec(&bunny.nodes[tetra.i4]) * scale + offset;
    let a = p2 - p1;
    let b = p3 - p1;
    let c = p4 - p1;
    let volume = Vector3f::dot(&a, &Vector3f::cross(&b, &c)) / 6.0;
    let mass = volume * density;
    let num_pars = mass / particle_mass;
    let par_volume = volume / num_pars;
    for _ in 0..num_pars as usize {
      let pos = random_point_in_tetra(p1, p2, p3, p4);
      world.put_particle(pos, initial_velocity, particle_mass, par_volume);
    }
  }

  // Make the world only show a portion
  world.only_show_random_portion(random_portion);

  // Generate progressbar and let it run
  let mut pb = ProgressBar::new(cycles);
  for _ in 0..cycles {
    pb.inc();
    world.step();
  }
  pb.finish_print("Finished");
}