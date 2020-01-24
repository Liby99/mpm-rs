extern crate msh_rs;
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

use msh_rs::TetrahedronMesh;
use specs::prelude::DispatcherBuilder;

type SpecsWorld = specs::prelude::World;

pub type Particle = specs::prelude::Entity;

pub struct WorldBuilder<'a, 'b> {
  grid: Grid,
  builder: DispatcherBuilder<'a, 'b>,
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

pub struct ParticlesHandle<'w, 'a, 'b> {
  world: &'w mut World<'a, 'b>,
  entities: Vec<Particle>,
}

impl<'w, 'a, 'b> ParticlesHandle<'w, 'a, 'b> {
  pub fn with<T: specs::prelude::Component + Clone>(self, c: T) -> Self {
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
  pub fn insert<T: specs::prelude::Component>(&mut self, p: Particle, c: T) {
    use specs::prelude::*;
    let mut store: WriteStorage<T> = self.world.system_data();
    store.insert(p, c).unwrap();
  }

  /// Remove a component of a given particle
  pub fn remove<T: specs::prelude::Component>(&mut self, p: Particle) {
    use specs::prelude::*;
    let mut store: WriteStorage<T> = self.world.system_data();
    store.remove(p);
  }

  /// Get the dimension of nodes of the grid
  pub fn dimension(&self) -> Vector3u {
    let grid = self.world.fetch::<Grid>();
    grid.dim
  }

  /// Get the h, the distance between a pair of neighbor node, of the grid
  pub fn h(&self) -> f32 {
    let grid = self.world.fetch::<Grid>();
    grid.h
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
    let num_nodes = (thickness / self.h()) as usize;
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

  /// Put a ball with a given center and radius. The ball will have a total mass of `mass`, and
  /// will contain `n` particles
  pub fn put_ball<'w>(&'w mut self, center: Vector3f, radius: f32, mass: f32, n: usize) -> ParticlesHandle<'w, 'a, 'b> {
    // Calculate individual mass and volume
    let total_volume = 1.333333 * std::f32::consts::PI * radius * radius * radius;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    let mut entities = vec![];
    for _ in 0..n {
      let pos = random_point_in_sphere(center, radius);
      let hdl = self.put_particle(pos, ind_mass).with(ParticleVolume(ind_volume));
      entities.push(hdl.entities[0]);
    }

    // Return the handle
    ParticlesHandle { world: self, entities }
  }

  /// Put a cube to the world, with the given `min` corner position and `max` corner
  /// position.
  pub fn put_cube<'w>(&'w mut self, min: Vector3f, max: Vector3f, mass: f32, n: usize) -> ParticlesHandle<'w, 'a, 'b> {
    assert!(min.x < max.x && min.y < max.y && min.z < max.z);

    // Calculate individual mass and volume
    let diff = max - min;
    let total_volume = diff.x * diff.y * diff.z;
    let ind_mass = mass / (n as f32);
    let ind_volume = total_volume / (n as f32);

    // Then add n particles
    let mut entities = vec![];
    for _ in 0..n {
      let pos = random_point_in_cube(min, max);
      let hdl = self.put_particle(pos, ind_mass).with(ParticleVolume(ind_volume));
      entities.push(hdl.entities[0]);
    }

    // Return the handle
    ParticlesHandle { world: self, entities }
  }

  /// Put a tetrahedron mesh to the world, given the `mesh` and `transf` as transform.
  /// You also need to provide a `density` (mass per unit cube) and the `par_mass` (mass
  /// per particle).
  pub fn put_tetra_mesh<'w>(
    &'w mut self,
    mesh: &TetrahedronMesh,
    transf: Transform3f,
    density: f32,
    par_mass: f32,
  ) -> ParticlesHandle<'w, 'a, 'b> {
    // All the added entities
    let mut entities = vec![];

    // Iterate through all tetrahedrons
    for tetra in &mesh.elems {
      let p1 = transf * msh_node_to_point(&mesh.nodes[tetra.i1]);
      let p2 = transf * msh_node_to_point(&mesh.nodes[tetra.i2]);
      let p3 = transf * msh_node_to_point(&mesh.nodes[tetra.i3]);
      let p4 = transf * msh_node_to_point(&mesh.nodes[tetra.i4]);
      let a = p2 - p1;
      let b = p3 - p1;
      let c = p4 - p1;
      let volume = Vector3f::dot(&a, &Vector3f::cross(&b, &c)) / 6.0;
      let mass = volume * density;
      let num_pars = mass / par_mass;
      let num_pars_usize = num_pars as usize;
      if num_pars_usize == 0 {
        let pos = random_point_in_tetra(p1.coords, p2.coords, p3.coords, p4.coords);
        let hdl = self.put_particle(pos, mass).with(ParticleVolume::new(volume));
        entities.push(hdl.entities[0])
      } else {
        let par_volume = volume / num_pars;
        for _ in 0..num_pars_usize {
          let pos = random_point_in_tetra(p1.coords, p2.coords, p3.coords, p4.coords);
          let hdl = self.put_particle(pos, par_mass).with(ParticleVolume::new(par_volume));
          entities.push(hdl.entities[0]);
        }
      }
    }

    // Return the handle
    ParticlesHandle { world: self, entities }
  }
}
