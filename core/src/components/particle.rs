use specs::prelude::*;

use crate::utils::*;

#[derive(Copy, Clone)]
pub struct ParticleMass(pub f32);

impl ParticleMass {
  pub fn get(&self) -> f32 {
    self.0
  }
}

impl Component for ParticleMass {
  type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct ParticleVolume(pub f32);

impl ParticleVolume {
  pub fn get(&self) -> f32 {
    self.0
  }
}

impl Component for ParticleVolume {
  type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct ParticlePosition(pub Vector3f);

impl ParticlePosition {
  pub fn get(&self) -> Vector3f {
    self.0
  }

  pub fn set(&mut self, p: Vector3f) {
    self.0 = p;
  }
}

impl Component for ParticlePosition {
  type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct ParticleVelocity(pub Vector3f);

impl ParticleVelocity {
  pub fn get(&self) -> Vector3f {
    self.0
  }

  pub fn set(&mut self, v: Vector3f) {
    self.0 = v;
  }
}

impl Component for ParticleVelocity {
  type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct ParticleDeformation {
  /// F
  pub deformation_gradient: Matrix3f,
  pub mu: f32,
  pub lambda: f32,
}

impl ParticleDeformation {
  /// E_0: Initial Young's Modulus
  /// nu: Poisson Ratio
  pub fn new(youngs_modulus: f32, nu: f32) -> Self {
    Self {
      deformation_gradient: Matrix3f::identity(),
      mu: youngs_modulus / (2.0 * (1.0 + nu)),
      lambda: youngs_modulus * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)),
    }
  }
}

impl Component for ParticleDeformation {
  type Storage = VecStorage<Self>;
}
