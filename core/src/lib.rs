extern crate msh_rs;
extern crate nalgebra as na;
extern crate poisson;
extern crate rand;
extern crate rand_distr;
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

use msh_rs::TetrahedronMesh;
use specs::prelude::DispatcherBuilder;

type SpecsWorld = specs::prelude::World;

pub type Particle = specs::prelude::Entity;

pub struct WorldBuilder<'a, 'b> {
  grid_size: Vector3f,
  grid_dx: f32,
  particle_density: f32,
  dt: f32,
  builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> WorldBuilder<'a, 'b> {
  pub fn new() -> Self {
    use specs::prelude::*;

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

    Self {
      grid_size: Vector3f::new(1.0, 1.0, 1.0),
      grid_dx: 0.02,
      particle_density: 2.0,
      dt: 0.001,
      builder: builder,
    }
  }

  pub fn with_size(mut self, size: Vector3f) -> Self {
    self.grid_size = size;
    self
  }

  pub fn with_dx(mut self, dx: f32) -> Self {
    self.grid_dx = dx;
    self
  }

  pub fn with_density(mut self, density: f32) -> Self {
    self.particle_density = density;
    self
  }

  pub fn with_dt(mut self, dt: f32) -> Self {
    self.dt = dt;
    self
  }

  pub fn with_system<T: for<'c> specs::RunNow<'c> + 'b>(mut self, system: T) -> Self {
    self.builder.add_thread_local(system);
    self
  }

  pub fn build(self) -> World<'a, 'b> {
    // First create a grid
    let x_dim = (self.grid_size.x / self.grid_dx) as usize;
    let y_dim = (self.grid_size.y / self.grid_dx) as usize;
    let z_dim = (self.grid_size.z / self.grid_dx) as usize;
    let grid_dim = Vector3u::new(x_dim, y_dim, z_dim);
    let grid = Grid::new(grid_dim, self.grid_dx);

    // Then generate the world & dispatcher
    use specs::prelude::WorldExt;
    let mut world = specs::prelude::World::new();
    let mut dispatcher = self.builder.build();
    dispatcher.setup(&mut world);

    // Set the world's grid to be grid
    *world.fetch_mut::<Grid>() = grid;

    // Set the world's Delta Time
    world.fetch_mut::<DeltaTime>().set(self.dt);

    // Return the world
    World {
      dispatcher,
      world,
      particle_density: self.particle_density,
    }
  }
}

pub struct ParticlesHandle<'w, 'a, 'b> {
  world: &'w mut World<'a, 'b>,
  entities: Vec<Particle>,
}

impl<'w, 'a, 'b> ParticlesHandle<'w, 'a, 'b> {
  pub fn first(self) -> Particle {
    self.entities[0]
  }

  pub fn with<T: specs::prelude::Component + Clone + Send + Sync>(self, c: T) -> Self {
    for &ent in &self.entities {
      self.world.insert(ent, c.clone());
    }
    self
  }

  pub fn hide_random_portion(self, percentage: f32) -> Self {
    for &ent in &self.entities {
      if random() > percentage {
        self.world.remove::<Hidden>(ent);
      } else {
        self.world.insert(ent, Hidden);
      }
    }
    self
  }

  pub fn each<F: Fn(&Particle, &mut World<'a, 'b>)>(self, f: F) -> Self {
    for ent in &self.entities {
      f(ent, self.world);
    }
    self
  }
}

pub struct World<'a, 'b> {
  pub world: SpecsWorld,
  pub particle_density: f32,
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
  pub fn hide_random_portion(&mut self, percentage: f32) {
    use specs::prelude::*;
    let (entities, poses): (Entities, ReadStorage<ParticlePosition>) = self.world.system_data();
    let mut hiddens: WriteStorage<Hidden> = self.world.system_data();
    for (entity, _) in (&entities, &poses).join() {
      if random() > percentage {
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

  /// Get the component of the given particle with the given type
  pub fn get<T: specs::prelude::Component + Clone>(&self, p: Particle) -> Option<T> {
    use specs::prelude::*;
    let store: ReadStorage<T> = self.world.system_data();
    store.get(p).map(T::clone)
  }

  /// Insert (will override if already presented) a component to a given particle
  pub fn insert<T: specs::prelude::Component + Send + Sync>(&mut self, p: Particle, c: T) {
    use specs::prelude::*;
    if let Some(mut store) = self.world.try_fetch_mut::<WriteStorage<T>>() {
      store.insert(p, c).unwrap();
    }
  }

  /// Remove a component of a given particle
  pub fn remove<T: specs::prelude::Component + Send + Sync>(&mut self, p: Particle) {
    use specs::prelude::*;
    if let Some(mut store) = self.world.try_fetch_mut::<WriteStorage<T>>() {
      store.remove(p);
    }
  }

  /// Get the dimension of nodes of the grid
  pub fn dimension(&self) -> Vector3u {
    let grid = self.world.fetch::<Grid>();
    grid.dim
  }

  /// Get the dx, the distance between a pair of neighbor node, of the grid
  pub fn dx(&self) -> f32 {
    let grid = self.world.fetch::<Grid>();
    grid.dx
  }

  /// Get the size of the grid
  pub fn size(&self) -> Vector3f {
    let grid = self.world.fetch::<Grid>();
    grid.size()
  }

  /// Put a boundary. Accept a callback function where given a node index, return an optional
  /// boundary. If `None` is returned from the callback, then nothing will be done; If `Some`
  /// is returned, then the boundary at that location will be updated
  pub fn put_boundary<F: Fn(Vector3u) -> Option<Boundary>>(&mut self, f: F) {
    let mut grid = self.world.fetch_mut::<Grid>();
    for node_index in grid.indices() {
      if let Some(b) = f(node_index) {
        grid.get_node_mut(node_index).boundary = b;
      }
    }
  }

  /// Put a wrapping boundary around the grid with a given thickness. It will accept a callback
  /// function `f`, which should accept a type of `Wall` and return a corresponding boundary.
  ///
  /// As a difference to `put_boundary`, no `None` would be accepted here.
  pub fn put_wrapping_boundary<F: Fn(Wall) -> Boundary>(&mut self, thickness: f32, f: F) {
    let dim = self.dimension();
    let num_nodes = (thickness / self.dx()) as usize;
    self.put_boundary(|node_index| {
      if node_index.x < num_nodes {
        Some(f(Wall::Left))
      } else if node_index.x > dim.x - num_nodes {
        Some(f(Wall::Right))
      } else if node_index.y < num_nodes {
        Some(f(Wall::Bottom))
      } else if node_index.y > dim.y - num_nodes {
        Some(f(Wall::Up))
      } else if node_index.z < num_nodes {
        Some(f(Wall::Back))
      } else if node_index.z > dim.z - num_nodes {
        Some(f(Wall::Front))
      } else {
        None
      }
    })
  }

  /// Put the `SetZero` boundary type to the boundary of the world within a given thickness
  pub fn put_sticky_boundary(&mut self, thickness: f32) {
    self.put_wrapping_boundary(thickness, |_| Boundary::Sticky)
  }

  /// Put the `Sliding` boundary type to the boundary of the world within a given thickness.
  /// The normal of the boundary will be automatically the normal of the box pointing inward.
  pub fn put_sliding_boundary(&mut self, thickness: f32) {
    self.put_wrapping_boundary(thickness, |w| Boundary::Sliding { normal: w.normal() })
  }

  /// Put the `Friction` boundary type to the boundary of the world within a given thickness.
  /// The normal of the boundary will be automatically the normal of the box pointing inward.
  /// The friction constant is given by the argument `mu`.
  pub fn put_friction_boundary(&mut self, thickness: f32, mu: f32) {
    self.put_wrapping_boundary(thickness, |w| Boundary::Friction { normal: w.normal(), mu })
  }

  /// Put a single particle at a given position with a given mass.
  pub fn put_particle<'w>(&'w mut self, pos: Vector3f, mass: f32) -> ParticlesHandle<'w, 'a, 'b> {
    use specs::prelude::*;
    let ent = self
      .world
      .create_entity()
      .with(ParticlePosition(pos))
      .with(ParticleVelocity(Vector3f::zeros()))
      .with(ParticleMass(mass))
      .build();
    ParticlesHandle {
      world: self,
      entities: vec![ent],
    }
  }

  /// Put a given region into the world with a transformation and a mass. The particles will be
  /// poisson sampled.
  pub fn put_region<'w, R>(&'w mut self, reg: R, transf: Similarity3f, mass: f32) -> ParticlesHandle<'w, 'a, 'b>
  where
    R: Region,
  {
    let mut entities = vec![];

    // Cache important data
    let radius = self.dx() / self.particle_density;
    let inv_transf = transf.inverse();

    // Get the bound transformed to the final position that the
    // object is going to be at
    let bb = reg.bound().transform(&transf);

    // Use that bounding box to generate poisson samples. The `sample`
    // here is at the world space
    for sample in bb.gen_poisson_samples(radius) {
      // Use inverse transform to get the sample in object local space
      let reg_ppos = inv_transf * Math::point_of_vector(&sample);

      // Check if the region contains the local space point, if is, then
      // add the world space position to the world
      if reg.contains(&reg_ppos) {
        let hdl = self.put_particle(sample, 0.0).with(ParticleVolume::new(radius.powi(3)));
        entities.push(hdl.first());
      }
    }

    // Finally calculate the mass being distributed to each particle
    let num_particles = entities.len() as f32;
    let ind_mass = mass / num_particles;
    for &ent in &entities {
      self.insert(ent, ParticleMass::new(ind_mass));
    }

    // Return the handle
    ParticlesHandle { world: self, entities }
  }

  /// A shortcut function adding a ball to the world
  pub fn put_ball<'w>(&'w mut self, center: Vector3f, radius: f32, mass: f32) -> ParticlesHandle<'w, 'a, 'b> {
    let reg = Sphere::new(radius);
    let translation = Translation3f::from(center);
    self.put_region(reg, na::convert(translation), mass)
  }

  /// A shortcut function adding a axis-aligned cube into the world
  pub fn put_cube<'w>(&'w mut self, min: Vector3f, max: Vector3f, mass: f32) -> ParticlesHandle<'w, 'a, 'b> {
    let size = max - min;
    let pos = min + size / 2.0;
    let reg = Cube::new(size);
    let translation = Translation3f::from(pos);
    self.put_region(reg, na::convert(translation), mass)
  }

  /// A shortcut function adding a tetrahedron mesh into the world
  pub fn put_tetra_mesh<'w>(
    &'w mut self,
    mesh: &TetrahedronMesh,
    transf: Similarity3f,
    mass: f32,
  ) -> ParticlesHandle<'w, 'a, 'b> {
    let reg = TetMesh::new(mesh);
    self.put_region(reg, transf, mass)
  }
}
