use super::*;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
  pub min: Point3f,
  pub max: Point3f,
}

impl BoundingBox {
  pub fn new_from_vec(min: Vector3f, max: Vector3f) -> Self {
    Self {
      min: Math::point_of_vector(&min),
      max: Math::point_of_vector(&max),
    }
  }

  pub fn new(min: Point3f, max: Point3f) -> Self {
    Self { min, max }
  }

  pub fn size(&self) -> Vector3f {
    self.max - self.min
  }

  pub fn transform(&self, transf: &Similarity3f) -> Self {
    let transl = transf.isometry.translation.vector;
    let rot = Matrix3f::from(transf.isometry.rotation);
    let (right, up, back) = (rot.column(0), rot.column(1), rot.column(2));
    let (xa, xb) = (right * self.min.x, right * self.max.x);
    let (ya, yb) = (up * self.min.y, up * self.max.y);
    let (za, zb) = (back * self.min.z, back * self.max.z);
    let rot_min = Math::component_min(&xa, &xb) + Math::component_min(&ya, &yb) + Math::component_min(&za, &zb);
    let rot_max = Math::component_max(&xa, &xb) + Math::component_max(&ya, &yb) + Math::component_max(&za, &zb);
    Self {
      min: Math::point_of_vector(&(rot_min * transf.scaling() + transl)),
      max: Math::point_of_vector(&(rot_max * transf.scaling() + transl)),
    }
  }
}
