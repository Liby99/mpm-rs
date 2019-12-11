use super::mpm::*;

pub struct Driver {
  pub world: World,
  pub step: u32,
  pub dt: f32,
}