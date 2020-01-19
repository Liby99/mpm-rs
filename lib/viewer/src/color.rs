use specs::prelude::*;

#[derive(Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

pub struct ParticleColor(pub Color);

impl Component for ParticleColor {
  type Storage = VecStorage<Self>;
}