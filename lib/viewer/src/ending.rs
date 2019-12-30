pub struct Ending(pub bool);

impl Default for Ending {
  fn default() -> Self {
    Self(false)
  }
}

impl Ending {
  pub fn is_ended(&self) -> bool {
    self.0
  }

  pub fn set_ended(&mut self) {
    self.0 = true;
  }
}