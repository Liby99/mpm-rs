use mpm_rs::*;
use mpm_viewer::*;

fn main() {
  let dt = 0.001;
  let world_size = Vector3f::new(10.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 200000.0;
  let poisson_ratio = 0.15;
  let boundary_thickness = 0.04;
  let boundary_fric_mu = 1.0;

  // Ball data
  let center = Vector3f::new(0.5, 0.5, 0.5);
  let velocity = Vector3f::new(8.0, 0.0, 0.0);
  let radius = 0.1;
  let mass = 20.0;
  let num_particles = 5000;
  let color_1 = Color::new(0.0, 1.0, 0.0);
  let color_2 = Color::new(0.0, 0.0, 1.0);

  // Initialize the world, use WindowSystem to visualize
  let mut world = WorldBuilder::new(world_size, grid_h)
    .with_system(WindowSystem::new())
    .build();

  // Set parameters
  world.set_dt(dt);

  // Put the boundary
  world.put_friction_boundary(boundary_thickness, boundary_fric_mu);

  // Put the balls
  world
    .put_ball(center, radius, mass, num_particles)
    .with(ParticleVelocity(velocity))
    .with(ParticleDeformation::new(youngs_modulus, poisson_ratio))
    .each(|&par, world| {
      let pos = world.get::<ParticlePosition>(par).unwrap().get();
      let (xp, yp, zp) = (pos.x >= center.x, pos.y >= center.y, pos.z >= center.z);
      let use_1 = (xp && yp && zp) || (xp && !yp && !zp) || (!xp && yp && !zp) || (!xp && !yp && zp);
      let color = if use_1 { color_1 } else { color_2 };
      world.insert(par, ParticleColor(color));
    });

  // Check the ending state determined by window system.
  // continue if not ended
  while world.not_ending() {
    world.step();
  }
}
