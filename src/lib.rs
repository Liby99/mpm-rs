extern crate specs;
extern crate nalgebra as na;
extern crate pbr;
extern crate rand;

pub mod utils;
pub mod components;
pub mod resources;
pub mod systems;

use utils::*;
use resources::*;
use systems::*;

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
    builder.add(ApplyElasticForceSystem, "apply_elasticity", &["apply_gravity"]);
    builder.add(GridF2VSystem, "grid_f2v", &["apply_gravity", "apply_elasticity"]);
    builder.add(GridSetBoundarySystem, "grid_set_boundary", &["grid_f2v"]);
    builder.add(EvolveDeformationSystem, "evolve_deformation", &["grid_set_boundary"]);
    builder.add(G2PSystem, "g2p", &["grid_set_boundary"]);
    builder.add_barrier();
    builder.add_thread_local(DumpSystem);

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

  pub fn set_output_dir(&mut self, output_dir: String) {
    self.world.fetch_mut::<OutputDirectory>().set(output_dir);
  }
}