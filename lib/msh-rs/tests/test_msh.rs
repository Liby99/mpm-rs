use msh_rs::*;

#[test]
fn test_msh_bunny() -> Result<(), msh_rs::Error> {
  let bunny = TetMesh::load("../../res/bunny.msh")?;
  assert_eq!(bunny.nodes.len(), 5946, "There should be 5946 nodes");
  assert_eq!(bunny.elems.len(), 23420, "There should be 23420 elements");
  Ok(())
}