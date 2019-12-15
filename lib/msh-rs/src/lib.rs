use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub enum Error {
  CannotReadFile,
  BadValue(usize),
  BadInteger(usize),
  BadElementType,
}

#[derive(Debug)]
pub struct Node {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug)]
pub enum Element {
  Tetrahedron { i1: u32, i2: u32, i3: u32, i4: u32 }
}

#[derive(Debug)]
pub enum ElementType {
  Tetra,
}

impl ElementType {
  pub fn from_u32(n: u32) -> Result<Self, Error> {
    match n {
      4 => Ok(Self::Tetra),
      _ => Err(Error::BadElementType)
    }
  }

  pub fn num_nodes_per_element(self) -> Result<u32, Error> {
    match self {
      Self::Tetra => Ok(4),
      // _ => Err(Error::BadElementType)
    }
  }
}

#[derive(Debug)]
pub struct Msh {
  nodes: Vec<Node>,
  elements: Vec<Element>,
}

pub fn check(buf: &Vec<u8>, id: &mut usize, val: u8) -> Result<(), Error> {
  *id += 1;
  if buf[*id] == val { Ok(()) } else { Err(Error::BadValue(*id)) }
}

pub fn check_array(buf: &Vec<u8>, start: &mut usize, val: &Vec<u8>) -> Result<(), Error> {
  for v in val {
    check(buf, start, *v)?;
  }
  Ok(())
}

pub fn check_str(buf: &Vec<u8>, start: &mut usize, s: &str) -> Result<(), Error> {
  check_array(buf, start, &Vec::from(s.as_bytes()))
}

pub fn load_ascii_u32(buf: &Vec<u8>, i: &mut usize, end: u8) -> Result<u32, Error> {
  let mut char_vec = Vec::new();
  while buf[*i] != end {
    char_vec.push(buf[*i]);
    *i += 1;
  }
  check(buf, i, end)?;
  let num_str = String::from_utf8_lossy(char_vec.as_slice());
  num_str.parse::<u32>().map_err(|_| Error::BadInteger(*i))
}

pub fn load_u32(buf: &Vec<u8>, start: &mut usize) -> Result<u32, Error> {
  let b1 = buf[*start] as u32;
  let b2 = buf[*start + 1] as u32;
  let b3 = buf[*start + 2] as u32;
  let b4 = buf[*start + 4] as u32;
  let n = (b4 << 24) | (b3 << 16) | (b2 << 8) | b1;
  *start += 4;
  Ok(n)
}

pub fn load_f64(buf: &Vec<u8>, start: &mut usize) -> Result<f64, Error> {
  let b1 = buf[*start] as u64;
  let b2 = buf[*start + 1] as u64;
  let b3 = buf[*start + 2] as u64;
  let b4 = buf[*start + 3] as u64;
  let b5 = buf[*start + 4] as u64;
  let b6 = buf[*start + 5] as u64;
  let b7 = buf[*start + 6] as u64;
  let b8 = buf[*start + 7] as u64;
  let n = (b8 << 56) | (b7 << 48) | (b6 << 40) | (b5 << 32) | (b4 << 24) | (b3 << 16) | (b2 << 8) | b1;
  let f = f64::from_bits(n);
  *start += 8;
  Ok(f)
}

pub fn load_node(buf: &Vec<u8>, start: &mut usize) -> Result<Node, Error> {
  let _ = load_u32(&buf, start)?; // Ignore index
  let x = load_f64(&buf, start)?;
  let y = load_f64(&buf, start)?;
  let z = load_f64(&buf, start)?;
  Ok(Node { x, y, z })
}

pub fn load(filename: String) -> Result<Msh, Error> {

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
  assert_eq!(binary_one, 1u32);

  // Parse header ending
  check_str(&buffer, &mut i, "$EndMeshFormat\n")?;

  // Parse start nodes
  check_str(&buffer, &mut i, "$Nodes\n")?;

  // Parse num nodes
  let num_nodes = load_ascii_u32(&buffer, &mut i, 0x0A)?; // End with '\n'

  // Parse nodes
  let mut nodes = Vec::new();
  for _ in 0..num_nodes {
    let node = load_node(&buffer, &mut i)?;
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
  let mut elements = Vec::new();
  while elem_read < num_elements {

    // Element header
    let elem_type = ElementType::from_u32(load_u32(&buffer, &mut i)?)?;
    let num_elems = load_u32(&buffer, &mut i)?;
    let num_tags = load_u32(&buffer, &mut i)?;

    // Get nodes per element
    let nodes_per_element = elem_type.num_nodes_per_element()?;
    assert_eq!(nodes_per_element, 4);

    // Go through the current elements
    for _ in 0..num_elems {
      let _ = load_u32(&buffer, &mut i)?; // Ignore element index

      // Don't care tags
      for _ in 0..num_tags {
        let _ = load_u32(&buffer, &mut i)?;
      }

      // Element values
      let i1 = load_u32(&buffer, &mut i)? - 1;
      let i2 = load_u32(&buffer, &mut i)? - 1;
      let i3 = load_u32(&buffer, &mut i)? - 1;
      let i4 = load_u32(&buffer, &mut i)? - 1;
      let element = Element::Tetrahedron { i1, i2, i3, i4 };
      elements.push(element);
    }

    // Increment the elem_read
    elem_read += num_elems;
  }

  Ok(Msh { nodes, elements })
}