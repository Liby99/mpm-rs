use specs::prelude::*;

#[derive(Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b }
  }
}

#[derive(Copy, Clone)]
pub struct ParticleColor(pub Color);

impl Component for ParticleColor {
  type Storage = VecStorage<Self>;
}
