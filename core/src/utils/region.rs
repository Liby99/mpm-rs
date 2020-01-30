use msh_rs::*;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::*;

pub trait Region {
  fn contains(&self, point: Point3f) -> bool;
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
}

type SpatialHashTableIndex = (usize, usize, usize);

struct SpatialHashTable<T: Hash + Eq + Clone> {
  pub dx: f32,
  pub table: HashMap<SpatialHashTableIndex, HashSet<T>>,
}

impl<T: Hash + Eq + Clone> SpatialHashTable<T> {
  pub fn new(dx: f32) -> Self {
    Self {
      dx,
      table: HashMap::new(),
    }
  }

  fn hash(&self, point: Point3f) -> SpatialHashTableIndex {
    let p = point.to_homogeneous();
    (
      (p.x / self.dx) as usize,
      (p.y / self.dx) as usize,
      (p.z / self.dx) as usize,
    )
  }

  pub fn put(&mut self, point: Point3f, item: T) {
    let idx = self.hash(point);
    self.table.entry(idx).or_insert(HashSet::new()).insert(item);
  }

  pub fn neighbors(&self, point: Point3f) -> HashSet<T> {
    let mut all = HashSet::new();
    let idx = self.hash(point);
    for i in idx.0 - 1..=idx.0 + 1 {
      for j in idx.1 - 1..=idx.1 + 1 {
        for k in idx.2 - 1..=idx.2 + 1 {
          let idx = (i, j, k);
          if let Some(items) = self.table.get(&idx) {
            for item in items {
              all.insert(item.clone());
            }
          }
        }
      }
    }
    all
  }
}

pub struct TetMesh {
  mesh: TetrahedronMesh,
  sht: SpatialHashTable<usize>,
}

impl TetMesh {
  pub fn new(mesh: TetrahedronMesh) -> Self {
    // First get the dx: divide the largest axis into 50 parts
    let (mut min, mut max) = (Vector3f::zeros(), Vector3f::zeros());
    for node in &mesh.nodes {
      let p = Self::point_of_node(node);
      min = Self::component_min(&p.coords, &min);
      max = Self::component_max(&p.coords, &max);
    }
    let dx = (max - min).argmax().1 / 50.0;

    // Then construct the spatial hash table
    let mut sht = SpatialHashTable::new(dx);
    for (i, elem) in mesh.elems.iter().enumerate() {
      let p1 = Self::point_of_node(&mesh.nodes[elem.i1]);
      let p2 = Self::point_of_node(&mesh.nodes[elem.i2]);
      let p3 = Self::point_of_node(&mesh.nodes[elem.i3]);
      let p4 = Self::point_of_node(&mesh.nodes[elem.i4]);
      sht.put(p1, i);
      sht.put(p2, i);
      sht.put(p3, i);
      sht.put(p4, i);
    }
    Self { mesh, sht }
  }

  fn component_max(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
    Vector3f::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z))
  }

  fn component_min(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
    Vector3f::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z))
  }

  fn point_of_node(node: &Node) -> Point3f {
    Point3f::new(node.x as f32, node.y as f32, node.z as f32)
  }
}

impl Region for TetMesh {
  fn contains(&self, point: Point3f) -> bool {
    for elem_index in self.sht.neighbors(point) {
      let elem = &self.mesh.elems[elem_index];
      let p1 = Self::point_of_node(&self.mesh.nodes[elem.i1]);
      let p2 = Self::point_of_node(&self.mesh.nodes[elem.i2]);
      let p3 = Self::point_of_node(&self.mesh.nodes[elem.i3]);
      let p4 = Self::point_of_node(&self.mesh.nodes[elem.i4]);
      let a = p2 - p1;
      let b = p3 - p1;
      let c = p4 - p1;
      let d = point - p1;
      let alpha = d.dot(&a);
      let beta = d.dot(&b);
      let gamma = d.dot(&c);
      if alpha + beta + gamma < 1.0 {
        return true;
      }
    }
    false
  }
}
