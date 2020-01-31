pub type Vector3f = na::Vector3<f32>;

pub type Vector3i = na::Vector3<i32>;

pub type Vector3u = na::Vector3<usize>;

pub type Point3f = na::Point3<f32>;

pub type Matrix3f = na::Matrix3<f32>;

pub type Quaternionf = na::Quaternion<f32>;

pub type Translation3f = na::Translation3<f32>;

pub type Rotation3f = na::Rotation3<f32>;

pub type Isometry3f = na::Isometry3<f32>;

pub type Similarity3f = na::Similarity3<f32>;

pub type Transform3f = na::Transform3<f32>;

pub struct Math;

impl Math {
  pub fn clamp(n: f32, low: f32, up: f32) -> f32 {
    f32::min(f32::max(n, low), up)
  }

  pub fn component_min(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
    v1.zip_map(v2, |x1, x2| x1.min(x2))
  }

  pub fn component_max(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
    v1.zip_map(v2, |x1, x2| x1.max(x2))
  }

  pub fn point_of_vector(v: &Vector3f) -> Point3f {
    Point3f::new(v.x, v.y, v.z)
  }

  pub fn vector_of_point(p: &Point3f) -> Vector3f {
    p.coords.clone()
  }
}
