pub struct DeltaTime(f32);

impl DeltaTime {
  pub fn get(&self) -> f32 {
    self.0
  }

  pub fn set(&mut self, dt: f32) {
    self.0 = dt;
  }
}

impl Default for DeltaTime {
  fn default() -> Self {
    Self(0.001)
  }
}
