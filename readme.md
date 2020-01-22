# Material Point Method in Rust

MPM algorithm implemented in Rust, empowered by [specs](https://specs.amethyst.rs).

## Use as a Library

``` rust
use mpm_rs::*;
use mpm_ply_dump::*;

fn main() {

  // Create a world
  let mut world = WorldBuilder::new(Vector3f::new(1.0, 1.0, 1.0), 0.02)
    .with_system(PlyDumpSystem::new("result", 3)) // Dump to "result"
    .build(); // Build the world

  // Create a boundary to the world with a thickness of 0.06
  world.put_boundary(0.06);

  // Create a ball in the world
  let center = Vector3f::new(0.5, 0.4, 0.5);
  let radius = 0.1;
  let mass = 10.0;
  let num_particles = 10000;
  let youngs_modulus = 10000.0;
  let nu = 0.2;
  world
    .put_ball(center, radius, mass, num_particles)
    .with(ParticleDeformation::new(youngs_modulus, nu));

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

Here we prefer `release` because it's so much faster than `debug`. `mickey_mouse` example
will output `.ply` files into the directory `result/mickey_mouse`. You can visualize the
result file using Houdini. You can check out more examples [here](examples/examples/).

As of an example simulation in a window visualized by [kiss3d](http://kiss3d.org), you can
run

```
$ cargo run --release --example multi_balls_viewer
```

## Folder structure

A pure MPM simulation framework is implemented in [`core/`](core/).

A `.msh` loader (tetrahedron mesh loader) is implemented here in [`lib/msh-rs`](lib/msh-rs/).

A `.ply` file exporter is implemented here in [`lib/ply-dump`](lib/ply-dump/).

A viewer is implemented here in [`lib/viewer`](lib/viewer).

Other examples are located here: [`examples/examples`](examples/examples).

## Behind the Hood

`mpm-rs` is empowered by [specs](https://specs.amethyst.rs), an Entity-Component-System
framework which suits best for doing all kinds of simulations. We structured the code base
such that all the Lagrangian particles are individual entities in ECS, and the Eularian
grid for doing integral and differentiation as a single resource `Grid`. Having not a single
structure particle but "Arrays-of-particle-properties" empowers us to have various kinds of
behaviors of different particles to perform flawlessly across the system. Some particles
might have deformation gradient, some might have evolving $J_p$. The Specs ECS can
automatically work on them simultaneously while providing lightning speed.