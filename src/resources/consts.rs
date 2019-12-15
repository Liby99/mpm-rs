pub struct Mu(f32);

impl Default for Mu {
  fn default() -> Self {
    Self(3846.153846)
  }
}

impl Mu {
  pub fn get(&self) -> f32 {
    self.0
  }

  pub fn set(&mut self, mu: f32) {
    self.0 = mu;
  }
}

pub struct Lambda(f32);

impl Default for Lambda {
  fn default() -> Self {
    Self(5769.230769)
  }
}

impl Lambda {
  pub fn get(&self) -> f32 {
    self.0
  }

  pub fn set(&mut self, lambda: f32) {
    self.0 = lambda;
  }
}