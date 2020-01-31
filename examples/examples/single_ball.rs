use mpm_rs::*;
use mpm_examples::*;

fn main() {
  run_example(
    Config {
      output_directory: "result/single_ball",
      world_dt: 0.001,
      dump_skip: 5,
      num_cycles: 1500,
      ..Default::default()
    },
    |world| {
      // Put the boundary
      world.put_sticky_boundary(0.04);

      // Put the ball
      world
        .put_ball(Vector3f::new(0.5, 0.4, 0.5), 0.1, 10.0)
        .with(ParticleDeformation::elastic(140000.0, 0.15));
    }
  )
}
