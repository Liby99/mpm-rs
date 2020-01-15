use crate::utils::*;

/// The boundary type information associated with each Node
#[derive(Copy, Clone, Debug)]
pub enum Boundary {
  /// Not a boundary
  None,

  /// When dealing with the node velocity, set it to zero
  SetZero,

  /// Remove the velocity component along surface normal
  Sliding {
    normal: Vector3f,
  },

  /// Remove the velocity component along surface normal
  /// and multiply the tangential velocity by the factor
  /// Useful for simulating (non-real) friction
  VelocityDiminish {
    normal: Vector3f,
    factor: f32,
  },

  Friction {
    normal: Vector3f,
    mu: f32,
  },
}

/// - - - - - - - - - - -
/// * * * * * * * * * * |
/// * * * * / \ * * * * |
/// * * * < - - > * * * |
/// * * * * \ / * * * * |
/// * * * * * * * * * * |
/// * * * * * * * * * * |

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
  /// - If the node has type `Boundary::Sliding`, it's velocity along the `normal` will be discarded
  pub fn set_boundary_velocity(&mut self) {
    match self.boundary {
      Boundary::None => {}
      Boundary::SetZero => {
        self.velocity = Vector3f::zeros();
      }
      Boundary::Sliding { normal } => {
        self.velocity -= Vector3f::dot(&self.velocity, &normal) * normal;
      }
      Boundary::VelocityDiminish { normal, factor } => {
        self.velocity -= Vector3f::dot(&self.velocity, &normal) * normal;
        self.velocity *= factor;
      }
      Boundary::Friction { normal, mu: _ } => {
        self.velocity -= Vector3f::dot(&self.velocity, &normal) * normal;
      }
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
          let uindex = Vector3u::new(node_index.x as usize, node_index.y as usize, node_index.z as usize);
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
  pub nodes: Vec<Node>,
}

impl Default for Grid {
  fn default() -> Self {
    Self::new(Vector3u::new(50, 50, 50), 0.02)
  }
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
  /// ## Example
  ///
  /// ``` rust
  /// # use mpm_rs::*;
  /// # let pos = Vector3f::new(0.5, 0.5, 0.5);
  /// # let grid = Grid::new(Vector3u::new(50, 50, 50), 0.02);
  /// for (node_index, weight, weight_gradient) in grid.neighbor_weights(pos) {
  ///   let node = grid.get_node(node_index);
  ///   // Do things with the node...
  /// }
  /// ```
  pub fn neighbor_weights(&self, pos: Vector3f) -> WeightIterator {
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
  /// ## Example
  ///
  /// ``` rust
  /// # use mpm_rs::*;
  /// # let grid = Grid::new(Vector3u::new(50, 50, 50), 0.02);
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
