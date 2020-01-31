use mpm_rs::*;
use mpm_examples::*;
use mpm_viewer::*;
use nalgebra as na;

struct Ball {
  center: Vector3f,
  velocity: Vector3f,
  radius: f32,
  mass: f32,
  color: Color,
}

fn main() {
  run_example(
    Config {
      world_dt: 0.001,
      output_directory: "result/bunny",
      num_cycles: 5000,
      dump_skip: 20,
      ..Default::default()
    },
    |world| {
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

      // Put the boundary
      world.put_friction_boundary(0.04, 1.4);

      // Put the balls
      for b in balls {
        let sph = Sphere::new(b.radius);
        let transl = Translation3f::from(b.center);
        world
          .put_region(sph, na::convert(transl), b.mass)
          .with(ParticleVelocity::new(b.velocity))
          .with(ParticleDeformation::elastic(140000.0, 0.15))
          .with(ParticleColor::new(b.color));
      }
    }
  )
}
