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
  /// F_E, elastic deformation gradient
  pub f_elastic: Matrix3f,

  /// F_P, plastic deformation gradient
  pub f_plastic: Matrix3f,

  /// mu_0, one of the initial Lame parameters
  pub mu: f32,

  /// lambda_0, the other one of the initial Lame parameters
  pub lambda: f32,

  /// Compression limit
  pub theta_c: f32,

  /// Stretch limit
  pub theta_s: f32,

  /// Hardening Factor, 0 for no hardening
  pub hardening: f32,
}

impl ParticleDeformation {
  /// E_0: Initial Young's Modulus
  /// nu: Poisson Ratio
  pub fn new(youngs_modulus: f32, nu: f32) -> Self {
    Self {
      f_elastic: Matrix3f::identity(),
      f_plastic: Matrix3f::identity(),
      mu: youngs_modulus / (2.0 * (1.0 + nu)),
      lambda: youngs_modulus * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)),
      theta_c: 1.0,
      theta_s: 1.0,
      hardening: 0.0,
    }
  }

  pub fn snow() -> Self {
    let youngs_modulus = 140000.0;
    let poisson_ratio = 0.2;
    Self {
      f_elastic: Matrix3f::identity(),
      f_plastic: Matrix3f::identity(),
      mu: youngs_modulus / (2.0 * (1.0 + poisson_ratio)),
      lambda: youngs_modulus * poisson_ratio / ((1.0 + poisson_ratio) * (1.0 - 2.0 * poisson_ratio)),
      theta_c: 0.025,
      theta_s: 0.0075,
      hardening: 10.0,
    }
  }
}

impl Component for ParticleDeformation {
  type Storage = VecStorage<Self>;
}
