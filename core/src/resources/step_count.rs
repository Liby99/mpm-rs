pub struct StepCount(usize);

impl Default for StepCount {
  fn default() -> Self {
    Self(0)
  }
}

impl StepCount {
  pub fn get(&self) -> usize {
    self.0
  }

  pub fn step(&mut self) {
    self.0 += 1;
  }
}
