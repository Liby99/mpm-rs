use super::math::*;

/// Compute the 1D Quadratic B spline weights
///
/// ### Arguments
///
/// * `x` - a value normalized in the _Index Space_
///
pub fn compute_weight_1d(x: f32) -> (Vector3f, usize) {
  let base_node = (x - 0.5).floor(); // Floating point version of index

  // Weight[0]
  let d0 = x - base_node + 1.0;
  let z = 1.5 - d0;
  let w0 = 0.5 * z * z;

  // Weight[1]
  let d1 = d0 - 1.0;
  let w1 = 0.75 - d1 * d1;

  // Weight[2]
  let d2 = 2.0 - d0;
  let zz = 1.5 - d2;
  let w2 = 0.5 * zz * zz;

  (Vector3f::new(w0, w1, w2), base_node as usize)
}