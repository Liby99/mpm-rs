use mpm_rs::*;
use mpm_viewer::*;
use mpm_examples::*;

fn main() {
  run_example(
    Config {
      world_size: Vector3f::new(10.0, 1.0, 1.0),
      world_dt: 0.001,
      output_directory: "result/rolling_snowball",
      num_cycles: 1000,
      dump_skip: 10,
      ..Default::default()
    },
    |world| {
      // Put the boundary
      world.put_friction_boundary(0.04, 1.0);

      // Put the balls
      let center = Vector3f::new(0.5, 0.5, 0.5);
      world
        .put_ball(center, 0.1, 20.0)
        .with(ParticleVelocity::new(Vector3f::new(8.0, 0.0, 0.0)))
        .with(ParticleDeformation::snow())
        .each(|&par, world| {
          let pos = world.get::<ParticlePosition>(par).unwrap().get();
          let (xp, yp, zp) = (pos.x >= center.x, pos.y >= center.y, pos.z >= center.z);
          let use_1 = (xp && yp && zp) || (xp && !yp && !zp) || (!xp && yp && !zp) || (!xp && !yp && zp);
          let color = if use_1 { Color::new(0.0, 1.0, 0.0) } else { Color::new(0.0, 0.0, 1.0) };
          world.insert(par, ParticleColor::new(color));
        });
    }
  )
}
