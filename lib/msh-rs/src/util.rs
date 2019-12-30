#[derive(Debug)]
pub enum Error {
  CannotReadFile,
  BadValue(usize),
  BadInteger(usize),
  BadElementType,
}

pub fn check(buf: &Vec<u8>, id: &mut usize, val: u8) -> Result<(), Error> {
  if buf[*id] == val {
    *id += 1;
    Ok(())
  } else {
    Err(Error::BadValue(*id))
  }
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
  let b4 = buf[*start + 3] as u32;
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
