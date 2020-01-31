use na::{allocator::*, *};

pub type Vector3f = Vector3<f32>;

pub type Vector3i = Vector3<i32>;

pub type Vector3u = Vector3<usize>;

pub type Point3f = Point3<f32>;

pub type Matrix3f = Matrix3<f32>;

pub type Quaternionf = Quaternion<f32>;

pub type Translation3f = Translation3<f32>;

pub type Rotation3f = Rotation3<f32>;

pub type Isometry3f = Isometry3<f32>;

pub type Similarity3f = Similarity3<f32>;

pub type Transform3f = Transform3<f32>;

pub struct Math;

impl Math {
  pub fn clamp(n: f32, low: f32, up: f32) -> f32 {
    f32::min(f32::max(n, low), up)
  }

  pub fn clamp_vec<D>(v: &VectorN<f32, D>, low: f32, up: f32) -> VectorN<f32, D>
  where
    D: Dim + DimName,
    DefaultAllocator: Allocator<f32, D>,
  {
    v.map(|x| Self::clamp(x, low, up))
  }

  pub fn component_min<D>(v1: &VectorN<f32, D>, v2: &VectorN<f32, D>) -> VectorN<f32, D>
  where
    D: Dim + DimName,
    DefaultAllocator: Allocator<f32, D>,
  {
    v1.zip_map(v2, |x1, x2| x1.min(x2))
  }

  pub fn component_max<D>(v1: &VectorN<f32, D>, v2: &VectorN<f32, D>) -> VectorN<f32, D>
  where
    D: Dim + DimName,
    DefaultAllocator: Allocator<f32, D>,
  {
    v1.zip_map(v2, |x1, x2| x1.max(x2))
  }

  pub fn point_of_vector(v: &Vector3f) -> Point3f {
    Point3f::new(v.x, v.y, v.z)
  }

  pub fn vector_of_point(p: &Point3f) -> Vector3f {
    p.coords.clone()
  }
}
