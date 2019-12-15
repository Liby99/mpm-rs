# Material Point Method in Rust

MPM algorithm implemented in Rust, empowered by [specs](https://specs.amethyst.rs).

## Use as a Library

``` rust
fn main() {

  // Create a world with size 1 * 1 * 1, and grid spacing 0.02
  let mut world = mpm_rs::World::new(Vector3f::new(1.0, 1.0, 1.0), 0.02);

  // Create a boundary to the world with a thickness of 0.06
  world.put_boundary(0.06);

  // Create a ball centered at (0.5, 0.4, 0.5) with radius 0.1
  // The ball will contain 10000 particles and weight 10.0
  world.put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.1, Vector3f::zeros(), 10.0, 10000);

  // Loop 500 steps
  for _ in 0..500 {
    world.step();
  }
}
```

## Compile and Run Tests

To compile and run test, do

```
$ cargo build --release
$ cargo run --release --bin mickey_mouse
```

Here we prefer `release` because it's so much faster than `debug`. You can also change
the name `mickey_mouse` to other test name located in [src/bin](src/bin).

## Behind the Hood

`mpm-rs` is empowered by [specs](https://specs.amethyst.rs), an Entity-Component-System
framework which suits best for doing all kinds of simulations. We structured the code base
such that all the Lagrangian particles are individual entities in ECS, and the Eularian
grid for doing integral and differentiation as a single resource `Grid`. Having not a single
structure particle but "Arrays-of-particle-properties" empowers us to have various kinds of
behaviors of different particles to perform flawlessly across the system. Some particles
might have deformation gradient, some might have evolving $J_p$. The Specs ECS can
automatically work on them simultaneously while providing lightning speed.