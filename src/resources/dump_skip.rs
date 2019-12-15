pub struct DumpSkip(usize);

impl Default for DumpSkip {
  fn default() -> Self {
    Self(1)
  }
}

impl DumpSkip {
  pub fn get(&self) -> usize {
    self.0
  }

  pub fn set(&mut self, skip: usize) {
    self.0 = skip;
  }

  pub fn need_dump(&self, step_count: usize) -> bool {
    step_count % self.0 == 0
  }
}