pub type Vector3f = na::Vector3<f32>;

pub type Vector3i = na::Vector3<i32>;

pub type Vector3u = na::Vector3<usize>;

pub type Point3f = na::Point3<f32>;

pub type Matrix3f = na::Matrix3<f32>;

pub type Quaternionf = na::Quaternion<f32>;

pub type Translation3f = na::Translation3<f32>;

pub type Similarity3f = na::Similarity3<f32>;

pub type Transform3f = na::Transform3<f32>;

pub fn clamp(n: f32, low: f32, up: f32) -> f32 {
  f32::min(f32::max(n, low), up)
}
