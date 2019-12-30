# Material Point Method in Rust

MPM algorithm implemented in Rust, empowered by [specs](https://specs.amethyst.rs).

## Use as a Library

``` rust
fn main() {

  // Create a world
  let mut world =
    mpm_rs::WorldBuilder::new( // Create a builder
      Vector3f::new(1.0, 1.0, 1.0), // Size of 1 x 1 x 1
      0.02 // grid spacing 0.02
    )
    .with_system(mpm_ply_dump::PlyDumpSystem::new("result", 3)) // Dump to "result" directory, 1 file per 3 steps
    .build(); // Build the world

  // Create a boundary to the world with a thickness of 0.06
  world.put_boundary(0.06);

  // Create a ball
  world.put_ball(
    Vector3f::new(0.5, 0.4, 0.5), // center of the ball
    0.1, // radius of the ball
    Vector3f::zeros(), // initial velocity of the ball
    10.0, // mass of the ball
    10000, // number of particles inside this ball
    10000.0, // young's modulus
    0.2, // nu
  );

  // Run 500 steps
  for _ in 0..500 {
    world.step(); // Step once
  }
}
```

## Compile and Run Examples

To compile and run examples, do

```
$ cargo build --release
$ cargo run --release --example mickey_mouse
```

Here we prefer `release` because it's so much faster than `debug`. You can also change
the name `mickey_mouse` to other test name located in [src/bin](src/bin).

`mickey_mouse` example will output `.ply` files into the directory `result/mickey_mouse`.
You can visualize the result file using Houdini.

Check out more examples [here](examples/examples/).

## Behind the Hood

`mpm-rs` is empowered by [specs](https://specs.amethyst.rs), an Entity-Component-System
framework which suits best for doing all kinds of simulations. We structured the code base
such that all the Lagrangian particles are individual entities in ECS, and the Eularian
grid for doing integral and differentiation as a single resource `Grid`. Having not a single
structure particle but "Arrays-of-particle-properties" empowers us to have various kinds of
behaviors of different particles to perform flawlessly across the system. Some particles
might have deformation gradient, some might have evolving $J_p$. The Specs ECS can
automatically work on them simultaneously while providing lightning speed.