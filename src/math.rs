use rand::Rng;

pub type Vector3f = na::Vector3<f32>;

pub type Vector3i = na::Vector3<i32>;

pub type Vector3u = na::Vector3<usize>;

pub type Matrix3f = na::Matrix3<f32>;

pub fn sample_point_in_sphere(center: Vector3f, radius: f32) -> Vector3f {
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
