# MPM-RS Core

This core library provides the simulation core functionalities. It also provides some auxiliary methods to initialize the world.

To construct an MPM world, you may want to use a `WorldBuilder`. After that you can configure the world by setting `dt`, setting boundary conditions, adding particles and so on.

``` rust
use mpm_rs::*;
use mpm_ply_dump::*;

fn main() {

  // Create a world
  let mut world = WorldBuilder::new(Vector3f::new(1.0, 1.0, 1.0), 0.02).build();

  // Set the dt
  world.set_dt(0.001);

  // Create a boundary to the world with a thickness of 0.06
  world.put_boundary(0.06);

  // Create a ball in the world
  let center = Vector3f::new(0.5, 0.4, 0.5);
  let (radius, mass, num_particles) = (0.1, 10.0, 10000);
  let (youngs_modulus, nu) = (10000.0, 0.2);
  world
    .put_ball(center, radius, mass, num_particles)
    .with(ParticleDeformation::new(youngs_modulus, nu));

  // Run 500 steps
  for _ in 0..500 {
    world.step(); // Step once
  }
}
```

## What this library does not provide

It does not visualize the particles. It does not perform output of any kind.