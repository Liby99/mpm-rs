extern crate nalgebra as na;

use mpm_rs::*;
use mpm_viewer::*;

struct Ball {
  center: Vector3f,
  velocity: Vector3f,
  radius: f32,
  mass: f32,
  color: Color,
}

fn main() {
  let dt = 0.001;
  let world_size = Vector3f::new(1.0, 1.0, 1.0);
  let grid_h = 0.02;
  let youngs_modulus = 140000.0;
  let nu = 0.15;
  let boundary_thickness = 0.04;
  let boundary_fric_mu = 1.4;

  // Balls data
  let balls = vec![
    Ball {
      center: Vector3f::new(0.5, 0.7, 0.5),
      velocity: Vector3f::new(3.0, 3.0, 5.0),
      radius: 0.1,
      mass: 10.0,
      color: Color::new(1.0, 0.0, 0.0),
    },
    Ball {
      center: Vector3f::new(0.3, 0.2, 0.9),
      velocity: Vector3f::new(-3.0, 5.0, -2.0),
      radius: 0.1,
      mass: 10.0,
      color: Color::new(0.0, 1.0, 0.0),
    },
    Ball {
      center: Vector3f::new(0.6, 0.4, 0.3),
      velocity: Vector3f::new(10.0, 2.0, 8.0),
      radius: 0.1,
      mass: 10.0,
      color: Color::new(0.0, 0.0, 1.0),
    },
  ];

  // Initialize the world, use WindowSystem to visualize
  let mut world = WorldBuilder::new()
    .with_size(world_size)
    .with_dx(grid_h)
    .with_dt(dt)
    .with_system(WindowSystem::new())
    .build();

  // Put the boundary
  world.put_friction_boundary(boundary_thickness, boundary_fric_mu);

  // Put the balls
  for b in balls {
    let sph = Sphere::new(b.radius);
    let transl = Translation3f::from(b.center);
    world
      .put_region(sph, na::convert(transl), b.mass)
      .with(ParticleVelocity::new(b.velocity))
      .with(ParticleDeformation::elastic(youngs_modulus, nu))
      .with(ParticleColor::new(b.color));
  }

  // Check the ending state determined by window system.
  // continue if not ended
  while world.not_ending() {
    world.step();
  }
}
