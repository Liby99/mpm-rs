pub struct OutputDirectory(String);

impl Default for OutputDirectory {
  fn default() -> Self {
    Self("out".to_string())
  }
}

impl OutputDirectory {
  pub fn get(&self) -> &String {
    &self.0
  }

  pub fn set(&mut self, dir: String) {
    self.0 = dir;
  }
}