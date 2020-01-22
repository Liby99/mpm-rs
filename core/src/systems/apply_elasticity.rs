use specs::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::utils::*;

/// Compute the derivative of Jacobian with respect to the matrix.
///
/// $$\frac{d J}{d F}$$
///
/// F = [ a b c ]
///     [ d e f ]
///     [ g h i ]
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
  let (a, b, c, d, e, f, g, h, i) = (m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8]);
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
fn get_rotation(f: Matrix3f) -> Matrix3f {
  let svd = f.svd(true, true);
  match (svd.u, svd.v_t) {
    (Some(u), Some(v_t)) => {
      // Invert the related U and Sigma component
      let u = if u.determinant() < 0.0 {
        Matrix3f::new(u[0], u[1], -u[2], u[3], u[4], -u[5], u[6], u[7], -u[8])
      } else {
        u
      };
      assert!(u.determinant() >= 0.0, "SVD det(U) < 0");

      // Invert the related V^T and Sigma component
      let v_t = if v_t.determinant() < 0.0 {
        Matrix3f::new(
          v_t[0], v_t[1], v_t[2], v_t[3], v_t[4], v_t[5], -v_t[6], -v_t[7], -v_t[8],
        )
      } else {
        v_t
      };
      assert!(v_t.determinant() >= 0.0, "SVD det(V^T) < 0");

      // Return U * V^T
      u * v_t
    }
    _ => panic!("Cannot decompose svd"),
  }
}

/// Find $\bold{P} = \frac{\partial \Phi}{\partial \bold{F}}$
fn fixed_corotated(f_e: Matrix3f, f_p: Matrix3f, mu_0: f32, lambda_0: f32, hardening: f32) -> Matrix3f {
  // Get J_E, J_P and R
  let j_e = f_e.determinant();
  let j_p = f_p.determinant();
  assert!(j_p >= 0.0); // !!!!!!!!
  let r_e = get_rotation(f_e);

  // Get mu and lambda from mu_0 and lambda_0
  let hardening_exp = hardening * (1.0 - j_p);
  let _hardening_factor = std::f32::consts::E.powf(hardening_exp);
  let mu = mu_0; // * hardening_factor; // TODO
  let lambda = lambda_0; // * hardening_factor; // TODO

  // Get dJ_E/dF_E
  let dje_dfe = dj_df(f_e);

  // Formula (5) in https://www.math.ucla.edu/~jteran/papers/SSCTS13.pdf
  2.0 * mu * (f_e - r_e) + lambda * (j_e - 1.0) * dje_dfe
}

pub struct ApplyElasticitySystem;

impl<'a> System<'a> for ApplyElasticitySystem {
  type SystemData = (
    Read<'a, DeltaTime>,
    Write<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    ReadStorage<'a, ParticleVolume>,
    ReadStorage<'a, ParticleDeformation>,
  );

  fn run(&mut self, (dt, mut grid, positions, volumes, deformations): Self::SystemData) {
    for (position, volume, def) in (&positions, &volumes, &deformations).join() {
      // Get the $hat{F_E_p}$
      let mut f_e_hat = Matrix3f::identity();
      for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node(node_index);
        f_e_hat += dt.get() * node.velocity_temp * grad_w.transpose();
      }
      f_e_hat = f_e_hat * def.f_elastic;

      // Use the fixed corotated model
      let stress = fixed_corotated(f_e_hat, def.f_plastic, def.mu, def.lambda, def.hardening);
      let vp0pft = volume.get() * stress * def.f_elastic.transpose();
      for (node_index, _, grad_w) in grid.neighbor_weights(position.get()) {
        let node = grid.get_node_mut(node_index);
        node.force -= vp0pft * grad_w;
      }
    }
  }
}
