use crate::utils::*;

pub struct Gravity(Vector3f);

impl Default for Gravity {
  fn default() -> Self {
    Self(Vector3f::new(0.0, -9.8, 0.0))
  }
}

impl Gravity {
  pub fn get(&self) -> Vector3f {
    self.0
  }
}
