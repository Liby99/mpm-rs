#[test]
fn test_msh_bunny() -> Result<(), msh_rs::Error> {
  let bunny = msh_rs::load("res/bunny.msh")?;
  assert_eq!(bunny.nodes.len(), 5946, "There should be 5946 nodes");
  assert_eq!(bunny.elements.len(), 23420, "There should be 23420 elements");
  Ok(())
}