use rand::Rng;
use super::*;

pub fn random_point_in_sphere(center: Vector3f, radius: f32) -> Vector3f {
  let mut rng = rand::thread_rng();
  loop {
    let x = rng.gen_range(-radius, radius);
    let y = rng.gen_range(-radius, radius);
    let z = rng.gen_range(-radius, radius);
    let v = Vector3f::new(x, y, z);
    if v.magnitude() <= radius {
      return center + v;
    }
  }
}

pub fn random_point_in_cube(min: Vector3f, max: Vector3f) -> Vector3f {
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(min.x, max.x);
  let y = rng.gen_range(min.y, max.y);
  let z = rng.gen_range(min.z, max.z);
  return Vector3f::new(x, y, z);
}