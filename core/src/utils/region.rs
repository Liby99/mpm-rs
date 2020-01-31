use msh_rs::*;

use super::*;

pub trait Region {
  /// Returns whether the region contains the given point
  fn contains(&self, point: Point3f) -> bool;

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
  fn contains(&self, point: Point3f) -> bool {
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
  fn contains(&self, point: Point3f) -> bool {
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
  d1: Vector3f,
  d2: Vector3f,
  d3: Vector3f,
}

impl Tetra {
  fn new(p1: Point3f, p2: Point3f, p3: Point3f, p4: Point3f) -> Self {
    Self {
      p1,
      d1: p2 - p1,
      d2: p3 - p1,
      d3: p4 - p1,
    }
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

  fn in_tetra(&self, point: Point3f, tetra: &Tetra) -> bool {
    let d = point - tetra.p1;
    let (a, b, c) = (d.dot(&tetra.d1), d.dot(&tetra.d2), d.dot(&tetra.d3));
    a > 0.0 && b > 0.0 && c > 0.0 && a + b + c < 1.0
  }
}

impl Region for TetMesh {
  fn contains(&self, point: Point3f) -> bool {
    for tetra in &self.tetras {
      if self.in_tetra(point, &tetra) {
        return true;
      }
    }
    false
  }

  fn bound(&self) -> BoundingBox {
    self.bb
  }
}
