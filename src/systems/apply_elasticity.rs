use specs::prelude::*;

use crate::utils::*;
use crate::resources::*;
use crate::components::*;

pub struct ApplyElasticitySystem;

impl<'a> System<'a> for ApplyElasticitySystem {
  type SystemData = (
    Read<'a, Mu>,
    Read<'a, Lambda>,
    Write<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    ReadStorage<'a, ParticleVolume>,
    ReadStorage<'a, ParticleDeformation>,
  );

  fn run(&mut self, (mu, lambda, mut grid, positions, volumes, deformations): Self::SystemData) {
    for (position, volume, deformation) in (&positions, &volumes, &deformations).join() {
      let stress = fixed_corotated(deformation.get(), mu.get(), lambda.get());
      let vp0pft = volume.get() * stress * deformation.get().transpose();
      for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node_mut(node_index);
        node.force -= vp0pft * grad_w;
      }
    }
  }
}

/// Compute the derivative of Jacobian with respect to the matrix.
///
/// F = [ a b c 1 0 0
///       d e f 0 1 0
///       g h i 0 0 1]
///
/// $$\frac{d J}{d F}$$
///
/// F = [ a b c
///       d e f
///       g h i ]
///
/// J = det(F) = aei + bfg + cdh - ceg - bdi - fha
///
/// dJ/da = ei - fh
/// dJ/db = fg - di
/// dJ/dc = dh - eg
/// dJ/dd = ch - bi
/// dJ/de = ai - cg
/// dJ/df = bg - ha
/// dJ/dg = bf - ce
/// dJ/dh = cd - fa
/// dJ/di = ae - bd
///
/// Lemma: $JF^{-T} = dJ/dF$
///
/// Proof:
///
/// JF^{-T} = J(F^{-1})^T
/// F^{-1} = 1/J * Adj(F)
/// J(F^{-1})^T = Adj(F)^T
/// Adj(F) = dJ/d(F^T)
/// Adj(F)^T = Adj(F^T) = dJ/dF
///
/// Hence $JF^{-T} = dJ/dF$
fn dj_df(m: Matrix3f) -> Matrix3f {
  let (a, b, c,
       d, e, f,
       g, h, i) = (m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8]);
  Matrix3f::new(
    e * i - f * h,
    f * g - d * i,
    d * h - e * g,
    c * h - b * i,
    a * i - c * g,
    b * g - a * h,
    b * f - c * e,
    c * d - a * f,
    a * e - b * d,
  )
}

/// Find $R = U \times V^T$ given $[U, \sigma, V] = svd(M)$ and $M$
///
/// $$R = U * V^T$$
fn get_rotation(deformation: Matrix3f) -> Matrix3f {
  let svd = deformation.svd(true, true);
  match (svd.u, svd.v_t) {
    (Some(u), Some(v_t)) => {
      // Invert the related U and Sigma component
      let u = if u.determinant() < 0.0 {
        Matrix3f::new(
          u[0], u[1], -u[2],
          u[3], u[4], -u[5],
          u[6], u[7], -u[8],
        )
      } else {
        u
      };
      assert!(u.determinant() >= 0.0);

      // Invert the related V^T and Sigma component
      let v_t = if v_t.determinant() < 0.0 {
        Matrix3f::new(
          v_t[0], v_t[1], v_t[2],
          v_t[3], v_t[4], v_t[5],
          -v_t[6], -v_t[7], -v_t[8],
        )
      } else {
        v_t
      };
      assert!(v_t.determinant() >= 0.0);

      // Return U * V^T
      u * v_t
    }
    _ => panic!("Cannot decompose svd"),
  }
}

/// Find $\bold{P} = \frac{\partial \Phi}{\partial \bold{F}}$
fn fixed_corotated(deformation: Matrix3f, mu: f32, lambda: f32) -> Matrix3f {
  let r = get_rotation(deformation);
  let j = deformation.determinant(); // J > 0
  assert!(j >= 0.0);
  let jf_t = dj_df(deformation);
  2.0 * mu * (deformation - r) + lambda * (j - 1.0) * jf_t
}