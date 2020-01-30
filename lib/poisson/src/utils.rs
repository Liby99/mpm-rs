use na::Dim;

pub fn dim<D: Dim>() -> usize {
  D::try_to_usize().unwrap()
}
