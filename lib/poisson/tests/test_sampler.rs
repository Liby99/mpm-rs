extern crate nalgebra as na;

use na::*;
use poisson::*;

type Vector2f = Vector2<f32>;

#[test]
fn sampler2d_default() {
  for sample in Sampler2f::new().generate() {
    assert!((0..2).all(|i| 0.0 <= sample[i] && sample[i] < 1.0));
  }
}

#[test]
fn sampler2d_uniform_size() {
  for sample in Sampler2f::new().with_size(Vector2f::new(5.0, 5.0)).generate() {
    assert!((0..2).all(|i| 0.0 <= sample[i] && sample[i] < 5.0))
  }
}

#[test]
fn sampler2d_arbitrary_size() {
  for sample in Sampler2f::new().with_size(Vector2f::new(3.0, 5.0)).generate() {
    assert!(0.0 <= sample[0] && sample[0] < 3.0);
    assert!(0.0 <= sample[1] && sample[1] < 5.0);
  }
}
