mod util;

use std::fs::File;
use std::io::prelude::*;

pub use util::Error;
use util::*;

/// Node is simply a "point", i.e. Vector3f, in `.msh` files
#[derive(Debug)]
pub struct Node {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Node {

  /// When loading from buffer, the format is [index, x, y, z].
  /// We don't need `index` here.
  pub fn from_buffer(buf: &Vec<u8>, i: &mut usize) -> Result<Self, Error> {
    let _ = load_u32(&buf, i)?; // Ignore index
    let x = load_f64(&buf, i)?;
    let y = load_f64(&buf, i)?;
    let z = load_f64(&buf, i)?;
    Ok(Node { x, y, z })
  }
}

/// A trait for generalizing element loading
pub trait Element: Sized {

  /// Should implement load element from buffer method
  fn from_buffer(buf: &Vec<u8>, i: &mut usize) -> Result<Self, Error>;

  /// Returns the number of nodes inside a single element
  fn num_nodes() -> u32;
}

#[derive(Debug)]
pub struct Tetrahedron {
  pub i1: usize,
  pub i2: usize,
  pub i3: usize,
  pub i4: usize,
}

impl Element for Tetrahedron {
  fn from_buffer(buffer: &Vec<u8>, i: &mut usize) -> Result<Self, Error> {
    let i1 = (load_u32(&buffer, i)? - 1) as usize;
    let i2 = (load_u32(&buffer, i)? - 1) as usize;
    let i3 = (load_u32(&buffer, i)? - 1) as usize;
    let i4 = (load_u32(&buffer, i)? - 1) as usize;
    Ok(Self { i1, i2, i3, i4 })
  }

  fn num_nodes() -> u32 {
    4
  }
}

#[derive(Debug)]
pub struct Triangle {
  pub i1: usize,
  pub i2: usize,
  pub i3: usize,
}

impl Element for Triangle {
  fn from_buffer(buffer: &Vec<u8>, i: &mut usize) -> Result<Self, Error> {
    let i1 = (load_u32(&buffer, i)? - 1) as usize;
    let i2 = (load_u32(&buffer, i)? - 1) as usize;
    let i3 = (load_u32(&buffer, i)? - 1) as usize;
    Ok(Self { i1, i2, i3 })
  }

  fn num_nodes() -> u32 {
    3
  }
}

#[derive(Debug)]
pub enum ElementType {
  Tetra,
  Tri,
}

impl ElementType {
  pub fn from_u32(n: u32) -> Result<Self, Error> {
    match n {
      2 => Ok(Self::Tri),
      4 => Ok(Self::Tetra),
      _ => Err(Error::BadElementType),
    }
  }

  pub fn num_nodes_per_element(self) -> Result<u32, Error> {
    match self {
      Self::Tetra => Ok(4),
      Self::Tri => Ok(3),
    }
  }
}

#[derive(Debug)]
pub struct ElemMesh<E: Element> {
  pub nodes: Vec<Node>,
  pub elems: Vec<E>,
}

pub type TetrahedronMesh = ElemMesh<Tetrahedron>;

pub type TriangleMesh = ElemMesh<Triangle>;

impl<E: Element> ElemMesh<E> {
  pub fn load(filename: &str) -> Result<Self, Error> {
    // Open file
    let mut file = File::open(filename).map_err(|_| Error::CannotReadFile)?;

    // Create buffer and read to buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| Error::CannotReadFile)?;

    let mut i = 0;

    // Check the header
    check_str(&buffer, &mut i, "$MeshFormat\n")?;

    // Parse version number
    let mut version_number = Vec::new();
    while buffer[i] != 0x20 {
      version_number.push(buffer[i]);
      i += 1;
    }
    check(&buffer, &mut i, 0x20)?; // space after version number

    // Parse file type
    check(&buffer, &mut i, 0x31)?; // file type should be '1'
    check(&buffer, &mut i, 0x20)?; // space after file type

    // Parse data size
    check(&buffer, &mut i, 0x38)?; // data size should be '8'
    check(&buffer, &mut i, 0x0A)?; // '\n' after data size

    // Parse binary one
    let binary_one = load_u32(&buffer, &mut i)?;
    assert_eq!(binary_one, 1u32, "Binary one should be equal to 1");

    // Parse header ending
    check_str(&buffer, &mut i, "$EndMeshFormat\n")?;

    // Parse start nodes
    check_str(&buffer, &mut i, "$Nodes\n")?;

    // Parse num nodes
    let num_nodes = load_ascii_u32(&buffer, &mut i, 0x0A)?; // End with '\n'

    // Parse nodes
    let mut nodes = Vec::new();
    for _ in 0..num_nodes {
      let node = Node::from_buffer(&buffer, &mut i)?;
      nodes.push(node);
    }

    // Parse end nodes
    check_str(&buffer, &mut i, "$EndNodes\n")?;

    // Parse start elements
    check_str(&buffer, &mut i, "$Elements\n")?;

    // Parse num elements
    let num_elements = load_ascii_u32(&buffer, &mut i, 0x0A)?; // End with '\n'

    // Parse elements
    let mut elem_read = 0;
    let mut elems = Vec::new();
    while elem_read < num_elements {
      // Element header
      let elem_type = ElementType::from_u32(load_u32(&buffer, &mut i)?)?;
      let num_elems = load_u32(&buffer, &mut i)?;
      let num_tags = load_u32(&buffer, &mut i)?;

      // Get nodes per element
      let nodes_per_element = elem_type.num_nodes_per_element()?;
      if nodes_per_element != E::num_nodes() {
        return Err(Error::BadElementType);
      }

      // Go through the current elements
      for _ in 0..num_elems {
        let _ = load_u32(&buffer, &mut i)?; // Ignore element index

        // Don't care tags
        for _ in 0..num_tags {
          let _ = load_u32(&buffer, &mut i)?;
        }

        // Element values
        let elem = E::from_buffer(&buffer, &mut i)?;
        elems.push(elem);
      }

      // Increment the elem_read
      elem_read += num_elems;
    }

    Ok(Self { nodes, elems })
  }
}
