# MPM-rs Examples

This package contains examples of `mpm-rs`. They use various auxiliary libraries to output the data to `.ply` files, render the particles to viewable windows, and load `.msh` files containing custom geometries.

## List of examples

- [Single Ball](examples/single_ball.rs)
  - A single ball bouncing to the ground.
  - When running, output to `result/single_ball` directory
  - `cargo run --release --example single_ball`
- [Mickey Mouse](examples/mickey_mouse.rs)
  - A mickey mouse (3 balls) bouncing to the ground.
  - Output to `result/mickey_mouse` directory
  - `cargo run --release --example mickey_mouse`