extern crate specs;
extern crate nalgebra as na;
extern crate pbr;
extern crate rand;
extern crate rayon;

pub mod utils;
pub mod components;
pub mod resources;
pub mod systems;

pub use utils::*;
pub use components::*;
pub use resources::*;
pub use systems::*;

pub struct World<'a, 'b> {
  dispatcher: specs::Dispatcher<'a, 'b>,
  world: specs::prelude::World,
}

impl<'a, 'b> World<'a, 'b> {
  pub fn new(size: Vector3f, h: f32) -> Self {
    use specs::prelude::*;

    // First create a grid
    let x_dim = (size.x / h) as usize;
    let y_dim = (size.y / h) as usize;
    let z_dim = (size.z / h) as usize;
    let grid_dim = Vector3u::new(x_dim, y_dim, z_dim);
    let grid = Grid::new(grid_dim, h);

    // Then initialize the specs::world
    let mut world = World::new();
    let mut builder = DispatcherBuilder::new();

    // Put all systems into the world
    builder.add(StepCounterSystem, "step_counter", &[]);
    builder.add(CleanGridSystem, "clean_grid", &[]);
    builder.add(P2GSystem, "p2g", &["clean_grid"]);
    builder.add(GridM2VSystem, "grid_m2v", &["p2g"]);
    builder.add(ApplyGravitySystem, "apply_gravity", &["grid_m2v"]);
    builder.add(ApplyElasticitySystem, "apply_elasticity", &["apply_gravity"]);
    builder.add(GridF2VSystem, "grid_f2v", &["apply_gravity", "apply_elasticity"]);
    builder.add(GridSetBoundarySystem, "grid_set_boundary", &["grid_f2v"]);
    builder.add(EvolveDeformationSystem, "evolve_deformation", &["grid_set_boundary"]);
    builder.add(G2PSystem, "g2p", &["grid_set_boundary"]);
    builder.add_thread_local(DumpSystem::default());

    // Build the world
    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);

    // Put our grid into the world
    *world.fetch_mut::<Grid>() = grid;

    // Return
    Self { dispatcher, world }
  }

  pub fn step(&mut self) {
    self.dispatcher.dispatch(&mut self.world);
  }

  pub fn set_dt(&mut self, dt: f32) {
    self.world.fetch_mut::<DeltaTime>().set(dt);
  }

  pub fn set_output_dir(&mut self, output_dir: &str) {
    self.world.fetch_mut::<OutputDirectory>().set(output_dir.to_string());
  }

  pub fn set_dump_skip(&mut self, dump_skip: usize) {
    self.world.fetch_mut::<DumpSkip>().set(dump_skip);
  }

  pub fn only_show_random_portion(&mut self, percentage: f32) {
    use specs::prelude::*;
    let (
      entities,
      poses,
      mut hiddens
    ): (
      Entities,
      ReadStorage<ParticlePosition>,
      WriteStorage<Hidden>
    ) = self.world.system_data();
    for (entity, _) in (&entities, &poses).join() {
      if random() < percentage {
        hiddens.remove(entity);
      } else {
        hiddens.insert(entity, Hidden).unwrap();
      }
    }
  }

  pub fn put_boundary(&mut self, thickness: f32) {
    let mut grid = self.world.fetch_mut::<Grid>();
    let dim = grid.dim;
    let num_nodes = (thickness / grid.h).ceil() as usize;
    for node_index in grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::SetZero
      } else if node_index.x > dim.x - num_nodes {
        Boundary::SetZero
      } else if node_index.y < num_nodes {
        Boundary::SetZero
      } else if node_index.y > dim.y - num_nodes {
        Boundary::SetZero
      } else if node_index.z < num_nodes {
        Boundary::SetZero
      } else if node_index.z > dim.z - num_nodes {
        Boundary::SetZero
      } else {
        Boundary::None
      };
      grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  pub fn put_sliding_boundary(&mut self, thickness: f32) {
    let mut grid = self.world.fetch_mut::<Grid>();
    let dim = grid.dim;
    let num_nodes = (thickness / grid.h).ceil() as usize;
    for node_index in grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::Sliding { normal: Vector3f::new(1.0, 0.0, 0.0) }
      } else if node_index.x > dim.x - num_nodes {
        Boundary::Sliding { normal: Vector3f::new(-1.0, 0.0, 0.0) }
      } else if node_index.y < num_nodes {
        Boundary::Sliding { normal: Vector3f::new(0.0, 1.0, 0.0) }
      } else if node_index.y > dim.y - num_nodes {
        Boundary::Sliding { normal: Vector3f::new(0.0, -1.0, 0.0) }
      } else if node_index.z < num_nodes {
        Boundary::Sliding { normal: Vector3f::new(0.0, 0.0, 1.0) }
      } else if node_index.z > dim.z - num_nodes {
        Boundary::Sliding { normal: Vector3f::new(0.0, 0.0, -1.0) }
      } else {
        Boundary::None
      };
      grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  pub fn put_vel_dim_boundary(&mut self, thickness: f32, factor: f32) {
    let mut grid = self.world.fetch_mut::<Grid>();
    let dim = grid.dim;
    let num_nodes = (thickness / grid.h).ceil() as usize;
    for node_index in grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(1.0, 0.0, 0.0), factor }
      } else if node_index.x > dim.x - num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(-1.0, 0.0, 0.0), factor }
      } else if node_index.y < num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(0.0, 1.0, 0.0), factor }
      } else if node_index.y > dim.y - num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(0.0, -1.0, 0.0), factor }
      } else if node_index.z < num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(0.0, 0.0, 1.0), factor }
      } else if node_index.z > dim.z - num_nodes {
        Boundary::VelocityDiminish { normal: Vector3f::new(0.0, 0.0, -1.0), factor }
      } else {
        Boundary::None
      };
      grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  pub fn put_particle(&mut self, pos: Vector3f, vel: Vector3f, m: f32, v: f32, youngs_modulus: f32, nu: f32) {
    use specs::prelude::*;
    self.world.create_entity()
      .with(ParticlePosition(pos))
      .with(ParticleVelocity(vel))
      .with(ParticleMass(m))
      .with(ParticleVolume(v))
      .with(ParticleDeformation::new(youngs_modulus, nu))
      .build();
  }

  pub fn put_ball(&mut self, center: Vector3f, radius: f32, vel: Vector3f, mass: f32, n: usize, youngs_modulus: f32, nu: f32) {
    // Calculate individual mass and volume
    let total_volume = 1.333333 * std::f32::consts::PI * radius * radius * radius;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    for _ in 0..n {
      let pos = random_point_in_sphere(center, radius);
      self.put_particle(pos, vel, ind_mass, ind_volume, youngs_modulus, nu);
    }
  }

  pub fn put_cube(&mut self, min: Vector3f, max: Vector3f, vel: Vector3f, mass: f32, n: usize, youngs_modulus: f32, nu: f32) {
    // Calculate individual mass and volume
    let diff = max - min;
    let total_volume = diff.x * diff.y * diff.z;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    for _ in 0..n {
      let pos = random_point_in_cube(min, max);
      self.put_particle(pos, vel, ind_mass, ind_volume, youngs_modulus, nu);
    }
  }
}