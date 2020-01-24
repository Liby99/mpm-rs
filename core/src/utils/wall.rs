use super::Vector3f;

pub enum Wall {
  Left,
  Right,
  Bottom,
  Up,
  Back,
  Front,
}

impl Wall {
  pub fn normal(&self) -> Vector3f {
    match self {
      Self::Left => Vector3f::new(1.0, 0.0, 0.0),
      Self::Right => Vector3f::new(-1.0, 0.0, 0.0),
      Self::Bottom => Vector3f::new(0.0, 1.0, 0.0),
      Self::Up => Vector3f::new(0.0, -1.0, 0.0),
      Self::Back => Vector3f::new(0.0, 0.0, 1.0),
      Self::Front => Vector3f::new(0.0, 0.0, -1.0),
    }
  }
}
