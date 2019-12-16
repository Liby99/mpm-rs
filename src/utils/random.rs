use rand::Rng;
use super::*;

pub fn random() -> f32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(0.0, 1.0)
}

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

pub fn random_point_in_tetra(p1: Vector3f, p2: Vector3f, p3: Vector3f, p4: Vector3f) -> Vector3f {
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(0.0, 1.0);
  let y = rng.gen_range(0.0, 1.0);
  let z = rng.gen_range(0.0, 1.0);
  let a = p2 - p1;
  let b = p3 - p1;
  let c = p4 - p1;
  p1 + (a * x) + (b * y) + (c * z)
}