extern crate nalgebra as na;
extern crate rand;
extern crate rayon;
extern crate specs;

pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use utils::*;

pub struct WorldBuilder<'a, 'b> {
  grid: Grid,
  builder: specs::prelude::DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> WorldBuilder<'a, 'b> {
  pub fn new(size: Vector3f, h: f32) -> Self {
    use specs::prelude::*;

    // First create a grid
    let x_dim = (size.x / h) as usize;
    let y_dim = (size.y / h) as usize;
    let z_dim = (size.z / h) as usize;
    let grid_dim = Vector3u::new(x_dim, y_dim, z_dim);
    let grid = Grid::new(grid_dim, h);

    // Then create basic builder
    let mut builder = DispatcherBuilder::new();

    // Put all systems into the world
    builder.add(StepCounterSystem, "step_counter", &[]);
    builder.add(CleanGridSystem, "clean_grid", &[]);
    builder.add(P2GSystem, "p2g", &["clean_grid"]);
    builder.add(GridM2VSystem, "grid_m2v", &["p2g"]);
    builder.add(ApplyGravitySystem, "apply_gravity", &["grid_m2v"]);
    builder.add(ApplyElasticitySystem, "apply_elasticity", &["apply_gravity"]);
    builder.add(ApplyFrictionSystem, "apply_friction", &["apply_elasticity"]);
    builder.add(GridF2VSystem, "grid_f2v", &["apply_gravity", "apply_elasticity"]);
    builder.add(GridSetBoundarySystem, "grid_set_boundary", &["grid_f2v"]);
    builder.add(EvolveDeformationSystem, "evolve_deformation", &["grid_set_boundary"]);
    builder.add(G2PSystem, "g2p", &["grid_set_boundary"]);

    Self { grid, builder }
  }

  pub fn with_system<T: for<'c> specs::RunNow<'c> + 'b>(mut self, system: T) -> Self {
    self.builder.add_thread_local(system);
    self
  }

  pub fn build(self) -> World<'a, 'b> {
    use specs::prelude::WorldExt;
    let mut world = specs::prelude::World::new();
    let mut dispatcher = self.builder.build();
    dispatcher.setup(&mut world);
    *world.fetch_mut::<Grid>() = self.grid;
    World { dispatcher, world }
  }
}

pub struct ParticlesHandle {
  world: *mut specs::prelude::World,
  entities: Vec<specs::prelude::Entity>,
}

impl ParticlesHandle {
  pub fn with<T: specs::prelude::Component + Clone>(self, c: T) -> Self {
    use specs::prelude::*;
    for &ent in &self.entities {
      unsafe {
        let mut storage: WriteStorage<T> = SystemData::fetch(&(*self.world));
        storage.insert(ent, c.clone()).unwrap();
      }
    }
    self
  }

  pub fn each<F>(self, f: F) -> Self
  where
    F: Fn(&specs::prelude::Entity, &mut specs::prelude::World),
  {
    for ent in &self.entities {
      unsafe {
        f(ent, &mut *self.world);
      }
    }
    self
  }
}

pub struct World<'a, 'b> {
  pub world: specs::prelude::World,
  dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> World<'a, 'b> {
  /// Step the world once
  pub fn step(&mut self) {
    self.dispatcher.dispatch(&mut self.world);
  }

  /// Set the dt of the world
  pub fn set_dt(&mut self, dt: f32) {
    self.world.fetch_mut::<DeltaTime>().set(dt);
  }

  /// Add `Hidden` marker to a random portion of all the present particles
  pub fn only_show_random_portion(&mut self, percentage: f32) {
    use specs::prelude::*;
    let (entities, poses, mut hiddens): (Entities, ReadStorage<ParticlePosition>, WriteStorage<Hidden>) =
      self.world.system_data();
    for (entity, _) in (&entities, &poses).join() {
      if random() < percentage {
        hiddens.remove(entity);
      } else {
        hiddens.insert(entity, Hidden).unwrap();
      }
    }
  }

  /// Get the number of particles in this world
  pub fn num_particles(&self) -> usize {
    use specs::prelude::*;
    let poses: ReadStorage<ParticlePosition> = self.world.system_data();
    let mut num = 0;
    for _ in (&poses).join() {
      num += 1;
    }
    num
  }

  /// Put the `SetZero` boundary type to the boundary of the world within a given thickness
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

  /// Put the `Sliding` boundary type to the boundary of the world within a given thickness.
  /// The normal of the boundary will be automatically the normal of the box pointing inward.
  pub fn put_sliding_boundary(&mut self, thickness: f32) {
    let mut grid = self.world.fetch_mut::<Grid>();
    let dim = grid.dim;
    let num_nodes = (thickness / grid.h).ceil() as usize;
    for node_index in grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(1.0, 0.0, 0.0),
        }
      } else if node_index.x > dim.x - num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(-1.0, 0.0, 0.0),
        }
      } else if node_index.y < num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(0.0, 1.0, 0.0),
        }
      } else if node_index.y > dim.y - num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(0.0, -1.0, 0.0),
        }
      } else if node_index.z < num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(0.0, 0.0, 1.0),
        }
      } else if node_index.z > dim.z - num_nodes {
        Boundary::Sliding {
          normal: Vector3f::new(0.0, 0.0, -1.0),
        }
      } else {
        Boundary::None
      };
      grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  /// Put the `Friction` boundary type to the boundary of the world within a given thickness.
  /// The normal of the boundary will be automatically the normal of the box pointing inward.
  /// The friction constant is given by the argument `mu`.
  pub fn put_friction_boundary(&mut self, thickness: f32, mu: f32) {
    let mut grid = self.world.fetch_mut::<Grid>();
    let dim = grid.dim;
    let num_nodes = (thickness / grid.h).ceil() as usize;
    for node_index in grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(1.0, 0.0, 0.0),
          mu,
        }
      } else if node_index.x > dim.x - num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(-1.0, 0.0, 0.0),
          mu,
        }
      } else if node_index.y < num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(0.0, 1.0, 0.0),
          mu,
        }
      } else if node_index.y > dim.y - num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(0.0, -1.0, 0.0),
          mu,
        }
      } else if node_index.z < num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(0.0, 0.0, 1.0),
          mu,
        }
      } else if node_index.z > dim.z - num_nodes {
        Boundary::Friction {
          normal: Vector3f::new(0.0, 0.0, -1.0),
          mu,
        }
      } else {
        Boundary::None
      };
      grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  pub fn put_particle(
    &mut self,
    pos: Vector3f,
    vel: Vector3f,
    m: f32,
    v: f32,
    youngs_modulus: f32,
    nu: f32,
  ) -> ParticlesHandle {
    use specs::prelude::*;
    let ent = self
      .world
      .create_entity()
      .with(ParticlePosition(pos))
      .with(ParticleVelocity(vel))
      .with(ParticleMass(m))
      .with(ParticleVolume(v))
      .with(ParticleDeformation::new(youngs_modulus, nu))
      .build();
    ParticlesHandle {
      world: &mut self.world,
      entities: vec![ent],
    }
  }

  pub fn put_ball(
    &mut self,
    center: Vector3f,
    radius: f32,
    vel: Vector3f,
    mass: f32,
    n: usize,
    youngs_modulus: f32,
    nu: f32,
  ) -> ParticlesHandle {
    // Calculate individual mass and volume
    let total_volume = 1.333333 * std::f32::consts::PI * radius * radius * radius;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    let mut entities = vec![];
    for _ in 0..n {
      let pos = random_point_in_sphere(center, radius);
      let hdl = self.put_particle(pos, vel, ind_mass, ind_volume, youngs_modulus, nu);
      entities.push(hdl.entities[0]);
    }

    // Return the handle
    ParticlesHandle {
      world: &mut self.world,
      entities,
    }
  }

  pub fn put_cube(
    &mut self,
    min: Vector3f,
    max: Vector3f,
    vel: Vector3f,
    mass: f32,
    n: usize,
    youngs_modulus: f32,
    nu: f32,
  ) -> ParticlesHandle {
    // Calculate individual mass and volume
    let diff = max - min;
    let total_volume = diff.x * diff.y * diff.z;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    let mut entities = vec![];
    for _ in 0..n {
      let pos = random_point_in_cube(min, max);
      let hdl = self.put_particle(pos, vel, ind_mass, ind_volume, youngs_modulus, nu);
      entities.push(hdl.entities[0]);
    }

    // Return the handle
    ParticlesHandle {
      world: &mut self.world,
      entities,
    }
  }
}
