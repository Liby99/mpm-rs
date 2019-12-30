# MPM-RS Core

This core library provides the simulation core functionalities. It also provides some auxiliary methods to initialize the world.

To construct an MPM world, you may want to use a `WorldBuilder`. After that you can configure the world by setting `dt`, setting boundary conditions, adding particles and so on.

``` rust
let world_size = Vector3f::new(1.0, 1.0, 1.0);
let cell_gap = 0.02;
let mut world = WorldBuilder::new(world_size, cell_gap).build();

// Set the `dt` of the `world`
world.set_dt(0.0001);

// Boundary condition: we have a boundary of thickness 0.04 (width of 2 nodes)
world.put_boundary(0.04);

// Add a deformable ball
let ball_center = Vector3f::new(0.5, 0.7, 0.5);
let ball_radius = 0.1;
let ball_velocity = Vector3f::zeros();
let ball_mass = 10.0;
let num_particles = 10000;
let ball_youngs_modulus = 10000.0;
let ball_nu = 0.2;
world.put_ball(
  ball_center,
  ball_radius,
  ball_velocity,
  ball_mass,
  num_particles,
  youngs_modulus,
  nu,
);
```

## What this library does not provide

It does not visualize the particles. It does not perform output of any kind.