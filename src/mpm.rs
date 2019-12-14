use std::fs::File;
use std::io::Write;

use super::math::*;

/// Particle inside MPM simulation
#[derive(Copy, Clone, Debug)]
pub struct Particle {
  /// the mass of the particle
  pub mass: f32,

  /// The volume of this particle
  pub volume: f32,

  /// the position of the particle
  pub position: Vector3f,

  /// the velocity of the particle
  pub velocity: Vector3f,

  /// $F_p$ deformation gradient
  pub deformation: Matrix3f,
}

impl Particle {
  /// Generate a new particle with given mass and position
  pub fn new(mass: f32, volume: f32, position: Vector3f) -> Self {
    let velocity = Vector3f::zeros();
    let deformation = Matrix3f::identity();
    Self {
      mass,
      volume,
      position,
      velocity,
      deformation,
    }
  }
}

/// The boundary type information associated with each Node
#[derive(Copy, Clone, Debug)]
pub enum Boundary {
  /// Not a boundary
  None,

  /// When dealing with the node velocity, set it to zero
  SetZero,

  /// Remove the velocity component along surface normal
  Surface { normal: Vector3f }, // TODO: Friction
}

/// The Node of the Grid
///
/// The `index` and `boundary` information of Node will be kept immutable, while
/// all the other information should be cleaned to `0` at the beginning of each step.
/// When initializing, we only need `index` and `boundary` information.
#[derive(Copy, Clone, Debug)]
pub struct Node {
  /// The mass of the node
  pub mass: f32,

  /// The lagrangian velocity at the node
  pub velocity: Vector3f,

  /// The momentum at the node
  pub momentum: Vector3f,

  /// The accumulated force at the node
  pub force: Vector3f,

  /// The type of boundary. Used to describe the boundary behavior of this node.
  /// should be default to `Boundary::None`
  pub boundary: Boundary,
}

impl Node {
  /// Generate a new node given the index of it in the Grid
  pub fn new() -> Self {
    Self {
      mass: 0.0,
      velocity: Vector3f::zeros(),
      momentum: Vector3f::zeros(),
      force: Vector3f::zeros(),
      boundary: Boundary::None,
    }
  }

  /// Clean the information of the node
  ///
  /// Should set everything but `index` and `boundary` to `0`
  pub fn clean(&mut self) {
    self.mass = 0.0;
    self.velocity = Vector3f::zeros();
    self.momentum = Vector3f::zeros();
    self.force = Vector3f::zeros();
  }

  /// Set the boundary velocity.
  ///
  /// Depending on the type of boundary this node possess, the velocity will be set accordingly:
  ///
  /// - If the node has type `Boundary::None`, it means this node is not on the boundary, and therefore
  ///   we do nothing to the velocity
  /// - If the node has type `Boundary::SetZero`, it means that any point touching this node will get
  ///   no velocity
  /// - If the node has type `Boundary::Surface`, it's velocity along the `normal` will be discarded
  pub fn set_boundary_velocity(&mut self) {
    match self.boundary {
      Boundary::SetZero => {
        self.velocity = Vector3f::zeros();
      }
      Boundary::Surface { normal } => {
        self.velocity -= Vector3f::dot(&self.velocity, &normal) * normal;
      }
      _ => {}
    }
  }
}

/// The weight iterator type storing essential information traversing
/// the neighboring nodes around a point
pub struct WeightIterator {
  h: f32,
  dim: Vector3u,
  base_node: Vector3i,
  curr_node: Vector3i,
  wx: Vector3f,
  wy: Vector3f,
  wz: Vector3f,
  dwx: Vector3f,
  dwy: Vector3f,
  dwz: Vector3f,
}

impl Iterator for WeightIterator {
  /// (Node Index, Weight, Weight Gradient)
  type Item = (Vector3u, f32, Vector3f);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let i = self.curr_node.x as usize;
      let j = self.curr_node.y as usize;
      let k = self.curr_node.z as usize;
      if i < 3 && j < 3 && k < 3 {
        // Get node index
        let node_index = self.base_node + self.curr_node;

        // Calculate weight
        let wi = self.wx[i];
        let wj = self.wy[j];
        let wk = self.wz[k];
        let wijk = wi * wj * wk;

        // Calculate weight gradient
        let dwijk_dx = self.dwx[i] * wj * wk / self.h;
        let dwijk_dy = wi * self.dwy[j] * wk / self.h;
        let dwijk_dz = wi * wj * self.dwz[k] / self.h;
        assert!(!dwijk_dx.is_nan() && !dwijk_dy.is_nan() && !dwijk_dz.is_nan());
        let grad_w = Vector3f::new(dwijk_dx, dwijk_dy, dwijk_dz);

        // Compute the `curr_node` for next step
        if self.curr_node.x == 2 {
          if self.curr_node.y == 2 {
            self.curr_node.z += 1;
            self.curr_node.x = 0;
            self.curr_node.y = 0;
          } else {
            self.curr_node.y += 1;
            self.curr_node.x = 0;
          }
        } else {
          self.curr_node.x += 1;
        }

        // Check if node is inside the grid
        // If not, then the loop will continue
        let x_in = 0 <= node_index.x && node_index.x < self.dim.x as i32;
        let y_in = 0 <= node_index.y && node_index.y < self.dim.y as i32;
        let z_in = 0 <= node_index.z && node_index.z < self.dim.z as i32;
        if x_in && y_in && z_in {
          let uindex = Vector3u::new(
            node_index.x as usize,
            node_index.y as usize,
            node_index.z as usize,
          );
          return Some((uindex, wijk, grad_w));
        }
      } else {
        return None;
      }
    }
  }
}

/// Node index iterator iterate through all the indices of a grid
pub struct NodeIndexIterator {
  dim: Vector3u,
  curr: Vector3u,
}

impl Iterator for NodeIndexIterator {
  type Item = Vector3u;

  fn next(&mut self) -> Option<Self::Item> {
    let x_in = self.curr.x < self.dim.x;
    let y_in = self.curr.y < self.dim.y;
    let z_in = self.curr.z < self.dim.z;
    if x_in && y_in && z_in {
      let result = self.curr;
      if self.curr.x == self.dim.x - 1 {
        if self.curr.y == self.dim.y - 1 {
          self.curr.z += 1;
          self.curr.y = 0;
          self.curr.x = 0;
        } else {
          self.curr.y += 1;
          self.curr.x = 0;
        }
      } else {
        self.curr.x += 1;
      }
      Some(result)
    } else {
      None
    }
  }
}

/// The Grid of Node in Lagrangian space
#[derive(Debug)]
pub struct Grid {
  /// The distance between each pair of neighbor nodes
  pub h: f32,

  /// Dimension vector; the number of nodes along each axis
  pub dim: Vector3u,

  /// Nodes Array
  nodes: Vec<Node>,
}

impl Grid {
  /// Create a new grid using `dimension` and `h`. All nodes will be initialized
  /// to initial `0` values.
  pub fn new(dim: Vector3u, h: f32) -> Self {
    let num_nodes = dim.x * dim.y * dim.z;
    let mut nodes = Vec::with_capacity(num_nodes);
    for _ in 0..num_nodes {
      nodes.push(Node::new());
    }
    Self { h, dim, nodes }
  }

  /// Clean the data of all the nodes
  pub fn clean(&mut self) {
    for node in &mut self.nodes {
      node.clean();
    }
  }

  /// Apply boundary constants to nodes
  pub fn set_boundary_velocities(&mut self) {
    for node in &mut self.nodes {
      node.set_boundary_velocity();
    }
  }

  /// Get the raw index inside the `nodes` array from `Vector3i`
  fn raw_index(&self, node_index: Vector3u) -> usize {
    let z_comp = self.dim.x * self.dim.y * node_index.z;
    let y_comp = self.dim.x * node_index.y;
    let x_comp = node_index.x;
    z_comp + y_comp + x_comp
  }

  /// Get the node using `Vector3i` node index
  pub fn get_node(&self, node_index: Vector3u) -> &Node {
    let index = self.raw_index(node_index);
    &self.nodes[index]
  }

  /// Get the mutable node using `Vector3i` node index
  pub fn get_node_mut(&mut self, node_index: Vector3u) -> &mut Node {
    let index = self.raw_index(node_index);
    &mut self.nodes[index]
  }

  /// Get 1d weight given position. Will normalize `pos` to index space.
  ///
  /// Returns the base node index, the weights of the three nodes, and the
  /// weight gradients of the three nodes.
  fn get_weight_1d(&self, pos: f32) -> (i32, Vector3f, Vector3f) {
    // `x` is normalized to index space
    let x = pos / self.h;

    // get the base node around `x`
    let base_node = (x - 0.5).floor();

    let mut w = Vector3f::zeros();
    let mut dw = Vector3f::zeros();

    let d0 = x - base_node;
    let z = 1.5 - d0;
    let z2 = z * z;
    w.x = 0.5 * z2;

    let d1 = d0 - 1.0;
    w.y = 0.75 - d1 * d1;

    let d2 = 1.0 - d1;
    let zz = 1.5 - d2;
    let zz2 = zz * zz;
    w.z = 0.5 * zz2;

    dw.x = -z;
    dw.y = -2.0 * d1;
    dw.z = zz;

    (base_node as i32, w, dw)
  }

  /// Iterate the neighbors. Will get `node_index`, `weight` and `weight_gradient`.
  /// for each neighbor node.
  ///
  /// ## Example usage
  ///
  /// ``` rust
  /// for (node_index, weight, weight_gradient) in grid.neighbor_weights(pos) {
  ///   let node = grid.get_node(node_index);
  ///   // Do things with the node...
  /// }
  /// ```
  fn neighbor_weights(&self, pos: Vector3f) -> WeightIterator {
    let (bnx, wx, dwx) = self.get_weight_1d(pos.x);
    let (bny, wy, dwy) = self.get_weight_1d(pos.y);
    let (bnz, wz, dwz) = self.get_weight_1d(pos.z);
    let h = self.h;
    let dim = self.dim;
    let base_node = Vector3i::new(bnx, bny, bnz);
    let curr_node = Vector3i::zeros();
    WeightIterator {
      h,
      dim,
      base_node,
      curr_node,
      wx,
      wy,
      wz,
      dwx,
      dwy,
      dwz,
    }
  }

  /// Iterate indices of the grid
  ///
  /// ## Example usage
  ///
  /// ``` rust
  /// for node_index in grid.indices() {
  ///   let node = grid.get_node(node_index);
  ///   // Do things with the node...
  /// }
  /// ```
  pub fn indices(&self) -> NodeIndexIterator {
    NodeIndexIterator {
      dim: self.dim,
      curr: Vector3u::zeros(),
    }
  }
}

/// The World containing all the simulation data of an MPM simulation
#[derive(Debug)]
pub struct World {
  /// The grid in Eularian space
  pub grid: Grid,

  /// The particles in Lagrangian space
  pub particles: Vec<Particle>,
}

impl World {
  /// Generate a new world given the `size` and the `h`
  pub fn new(size: Vector3f, h: f32) -> Self {
    let x_dim = (size.x / h) as usize;
    let y_dim = (size.y / h) as usize;
    let z_dim = (size.z / h) as usize;
    let grid = Grid::new(Vector3u::new(x_dim, y_dim, z_dim), h);
    let particles = vec![];
    Self { grid, particles }
  }

  /// Put a `SetZero` boundary around the world.
  pub fn put_zero_boundary(&mut self, thickness: f32) {
    let dim = self.grid.dim;
    let num_nodes = (thickness / self.grid.h).ceil() as usize;
    for node_index in self.grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::SetZero
      } else if node_index.x > dim.x - num_nodes {
        Boundary::SetZero
      } else if node_index.y < num_nodes {
        Boundary::SetZero
      } else if node_index.y > dim.y - num_nodes {
        Boundary::SetZero
      } else if node_index.z < num_nodes {
        Boundary::SetZero
      } else if node_index.z > dim.z - num_nodes {
        Boundary::SetZero
      } else {
        Boundary::None
      };
      self.grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  /// Put a surface boundary around the world. Each side will have normal pointing
  /// towards inside. The boundary thickness is given by the argument `thickness`.
  ///
  /// ## Arguments
  ///
  /// - `thickness` the thickness of the boundary, in real space
  pub fn put_surface_boundary(&mut self, thickness: f32) {
    let dim = self.grid.dim;
    let num_nodes = (thickness / self.grid.h).ceil() as usize;
    for node_index in self.grid.indices() {
      let boundary = if node_index.x < num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(1.0, 0.0, 0.0),
        }
      } else if node_index.x > dim.x - num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(-1.0, 0.0, 0.0),
        }
      } else if node_index.y < num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(0.0, 1.0, 0.0),
        }
      } else if node_index.y > dim.y - num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(0.0, -1.0, 0.0),
        }
      } else if node_index.z < num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(0.0, 0.0, 1.0),
        }
      } else if node_index.z > dim.z - num_nodes {
        Boundary::Surface {
          normal: Vector3f::new(0.0, 0.0, -1.0),
        }
      } else {
        Boundary::None
      };
      self.grid.get_node_mut(node_index).boundary = boundary;
    }
  }

  /// Create a ball filled with particles
  ///
  /// ## Arguments
  ///
  /// - `center` The center of the ball
  /// - `radius` The radius of the ball
  /// - `num_particles` The number of particles needed inside that ball
  /// - `total_mass` The total mass of the whole ball. The mass will be distributed onto each particle
  ///   uniformly.
  pub fn put_ball(&mut self, center: Vector3f, radius: f32, num_particles: usize, total_mass: f32) {
    let total_volume = 1.333333 * std::f32::consts::PI * radius * radius * radius;
    let ind_mass = total_mass / (num_particles as f32);
    let ind_volume = total_volume / (num_particles as f32);
    for _ in 0..num_particles {
      let pos = sample_point_in_sphere(center, radius);
      let par = Particle::new(ind_mass, ind_volume, pos);
      self.particles.push(par);
    }
  }

  /// 1. Clean the grid
  fn clean_grid(&mut self) {
    self.grid.clean();
  }

  /// 2. Particle -> Grid Transfer
  ///
  /// Assumes that when calling this function, the grid is already cleaned up (e.g. all
  /// temporary variables like mass, velocity, momentum and force are set to `0`)
  ///
  /// Transfer the particle mass and weight to the Grid by accumulating mass and momentum
  /// on each node.
  fn p2g(&mut self) {
    for par in &self.particles {
      for (node_index, weight, _) in self.grid.neighbor_weights(par.position) {
        let node = self.grid.get_node_mut(node_index);
        node.mass += par.mass * weight;
        node.momentum += par.mass * par.velocity * weight;
      }
    }
  }

  /// 3. Transfer the `momentum` into `velocity` for each grid node
  ///
  /// If a node has mass `0.0`, then the velocity will be automatically zero; otherwise,
  /// the velocity of node is simply $frac{momentum}{mass}$.
  fn grid_momentum_to_velocity(&mut self) {
    for node in &mut self.grid.nodes {
      // Check node mass. If 0, then directly set the velocity of node to zero
      if node.mass == 0.0 {
        node.velocity = Vector3f::zeros();
      } else {
        node.velocity = node.momentum / node.mass;
      }
    }
  }

  /// 4.1. Apply gravity force onto each of the grid node
  fn apply_gravity(&mut self) {
    let g = Vector3f::new(0.0, -9.8, 0.0);
    for node in &mut self.grid.nodes {
      node.force += g * node.mass;
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
  /// Note:
  ///
  /// Calculate $JF^{-T} = J(F^{-1})^T$
  ///
  /// F^{-1} = 1/J * Adj(F)
  /// J(F^{-1})^T = Adj(F)^T
  /// Adj(F) = dJ/d(F^T)
  /// Adj(F)^T = Adj(F^T) = dJ/dF
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
  fn polar_svd_r(m: Matrix3f) -> Matrix3f {
    let svd = m.svd(true, true);
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
    let r = Self::polar_svd_r(deformation);
    let j = deformation.determinant(); // J > 0
    assert!(j >= 0.0);
    let jf_t = Self::dj_df(deformation);
    2.0 * mu * (deformation - r) + lambda * (j - 1.0) * jf_t
  }

  /// 4.2. Apply elastic force onto each grid node.
  ///
  /// Also needs to take into account the `f` of each particle.
  ///
  /// # TODO: Make the constants per-particle
  fn apply_elastic_force(&mut self) {
    for par in &mut self.particles {
      let stress = Self::fixed_corotated(par.deformation, 3846.153846, 5769.230769);
      let vp0pft = par.volume * stress * par.deformation.transpose();
      for (node_index, _, grad_w) in self.grid.neighbor_weights(par.position) {
        let node = self.grid.get_node_mut(node_index);
        node.force -= vp0pft * grad_w;
      }
    }
  }

  /// 4.3. Transfer the force on each grid node into their velocity
  ///
  /// $\vec{a} = \frac{f}{m}$
  /// $\vec{v} = \delta t * \vec{a}$
  fn grid_force_to_velocity(&mut self, dt: f32) {
    for node in &mut self.grid.nodes {
      if node.mass != 0.0 {
        node.velocity += node.force / node.mass * dt;
      }
    }
  }

  /// 5. Apply boundary condition to the grid nodes
  fn set_boundary_velocities(&mut self) {
    self.grid.set_boundary_velocities();
  }

  /// 6. Evolve the particle deformation
  fn evolve_particle_deformation(&mut self, dt: f32) {
    for par in &mut self.particles {
      let mut grad_vp = Matrix3f::zeros();
      for (node_index, _, grad_w) in self.grid.neighbor_weights(par.position) {
        let node = self.grid.get_node(node_index);
        grad_vp += node.velocity * grad_w.transpose();
      }
      par.deformation *= Matrix3f::identity() + dt * grad_vp;
    }
  }

  /// 7. Grid to Particle transfer
  ///
  /// Will clear the particle velocity; accumulate the velocity of neighbor
  /// nodes onto the particle; and finally move the particle forward
  fn g2p(&mut self, dt: f32) {
    for par in &mut self.particles {

      // First clear the velocity
      par.velocity = Vector3f::zeros();

      // Accumulate velocities from neighbor nodes
      for (node_index, weight, _) in self.grid.neighbor_weights(par.position) {
        let node = self.grid.get_node(node_index);
        par.velocity += weight * node.velocity;
      }

      // Finally move the particles using the velocity
      par.position += dt * par.velocity;
    }
  }

  /// Step the world forward by `dt` (delta time).
  pub fn step(&mut self, dt: f32) {
    // 1. Clean grid data by zeroing out everything.
    self.clean_grid();

    // 2. Transfer particles to grid, will give mass and momentum to grid nodes
    self.p2g();

    // 3. Go over all grid nodes, convert momentum to velocity
    self.grid_momentum_to_velocity();

    // 4. Apply forces on grid
    self.apply_gravity(); // 4.1.
    self.apply_elastic_force(); // 4.2.

    // 4.3. Turn force into velocity
    self.grid_force_to_velocity(dt);

    // 5. Clean the boundary
    self.set_boundary_velocities();

    // 6. Evolve particle deformation
    self.evolve_particle_deformation(dt);

    // 7. Interpolate new velocity back to particles, and move the particles
    self.g2p(dt);
  }

  /// Dump the particles in current state to a file in `.POLY` format.
  pub fn dump(&self, filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(b"POINTS\n")?;
    for (i, par) in self.particles.iter().enumerate() {
      let pos = par.position;
      let line = format!("{}: {} {} {}\n", i + 1, pos.x, pos.y, pos.z);
      file.write(line.as_bytes())?;
    }
    file.write(b"POLYS\nEND\n")?;
    Ok(())
  }
}
