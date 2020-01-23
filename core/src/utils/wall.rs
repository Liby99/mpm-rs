use super::Vector3f;

pub enum Wall {
  Up,
  Bottom,
  Left,
  Right,
  Front,
  Back,
}

impl Wall {
  pub fn normal(&self) -> Vector3f {
    match self {
      Self::Up => Vector3f::new(0.0, -1.0, 0.0),
      Self::Bottom => Vector3f::new(0.0, 1.0, 0.0),
      Self::Left => Vector3f::new(1.0, 0.0, 0.0),
      Self::Right => Vector3f::new(-1.0, 0.0, 0.0),
      Self::Front => Vector3f::new(0.0, 0.0, 1.0),
      Self::Back => Vector3f::new(0.0, 0.0, -1.0),
    }
  }
}