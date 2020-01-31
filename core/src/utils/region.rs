use msh_rs::*;

use super::*;

pub trait Region {
  /// Returns whether the region contains the given point
  fn contains(&self, point: &Point3f) -> bool;

  /// Returns the axis-aligned bounding box of the region
  fn bound(&self) -> BoundingBox;
}

#[derive(Copy, Clone, Debug)]
pub struct Cube {
  pub size: Vector3f,
  half_size: Vector3f,
}

impl Cube {
  pub fn new(size: Vector3f) -> Self {
    Self {
      size,
      half_size: size / 2.0,
    }
  }
}

impl Region for Cube {
  fn contains(&self, point: &Point3f) -> bool {
    let p = point.coords;
    let x_contains = p.x.abs() < self.half_size.x;
    let y_contains = p.y.abs() < self.half_size.y;
    let z_contains = p.z.abs() < self.half_size.z;
    x_contains && y_contains && z_contains
  }

  fn bound(&self) -> BoundingBox {
    BoundingBox::new_from_vec(-self.half_size, self.half_size)
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  pub radius: f32,
}

impl Sphere {
  pub fn new(radius: f32) -> Self {
    Self { radius }
  }
}

impl Region for Sphere {
  fn contains(&self, point: &Point3f) -> bool {
    let dist = point.coords.magnitude();
    dist < self.radius
  }

  fn bound(&self) -> BoundingBox {
    let p = Point3f::new(self.radius, self.radius, self.radius);
    BoundingBox::new(-p, p)
  }
}

pub struct TetMesh {
  tetras: Vec<Tetra>,
  bb: BoundingBox,
}

struct Tetra {
  p1: Point3f,
  p2: Point3f,
  p3: Point3f,
  p4: Point3f,
}

impl Tetra {
  fn new(p1: Point3f, p2: Point3f, p3: Point3f, p4: Point3f) -> Self {
    Self { p1, p2, p3, p4 }
  }

  fn same_side(p1: &Point3f, p2: &Point3f, p3: &Point3f, p4: &Point3f, p: &Point3f) -> bool {
    let normal = (p2 - p1).cross(&(p3 - p1));
    let dot_p4 = normal.dot(&(p4 - p1));
    let dot_p = normal.dot(&Math::vector_of_point(p));
    dot_p.is_sign_positive() == dot_p4.is_sign_positive()
  }

  fn contains(&self, point: &Point3f) -> bool {
    let ss1 = Self::same_side(&self.p1, &self.p2, &self.p3, &self.p4, point);
    let ss2 = Self::same_side(&self.p2, &self.p3, &self.p4, &self.p1, point);
    let ss3 = Self::same_side(&self.p3, &self.p4, &self.p1, &self.p2, point);
    let ss4 = Self::same_side(&self.p4, &self.p1, &self.p2, &self.p3, point);
    ss1 && ss2 && ss3 && ss4
  }
}

impl TetMesh {
  pub fn new(mesh: &TetrahedronMesh) -> Self {
    let mut points = Vec::with_capacity(mesh.nodes.len());
    let mut tetras = Vec::with_capacity(mesh.elems.len());

    // First get the points, and cache all the points
    let v0 = Self::vector_of_node(&mesh.nodes[0]);
    let (mut min, mut max) = (v0, v0);
    for node in &mesh.nodes {
      let p = Self::point_of_node(node);
      min = Math::component_min(&p.coords, &min);
      max = Math::component_max(&p.coords, &max);
      points.push(p);
    }
    let bb = BoundingBox::new_from_vec(min, max);

    // Then cache all the tetrahedrons
    for elem in &mesh.elems {
      tetras.push(Tetra::new(
        points[elem.i1],
        points[elem.i2],
        points[elem.i3],
        points[elem.i4],
      ));
    }

    // Return the mesh
    Self { tetras, bb }
  }

  fn point_of_node(node: &Node) -> Point3f {
    Point3f::new(node.x as f32, node.y as f32, node.z as f32)
  }

  fn vector_of_node(node: &Node) -> Vector3f {
    Vector3f::new(node.x as f32, node.y as f32, node.y as f32)
  }
}

impl Region for TetMesh {
  fn contains(&self, point: &Point3f) -> bool {
    for tetra in &self.tetras {
      if tetra.contains(point) {
        return true;
      }
    }
    false
  }

  fn bound(&self) -> BoundingBox {
    self.bb
  }
}
