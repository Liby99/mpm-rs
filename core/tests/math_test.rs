use mpm_rs::*;

#[test]
fn vector_point_transf_1() {
  let p = Point3f::new(1.0, 1.0, 1.0);
  let v_of_p = Math::vector_of_point(&p);
  println!("Point3f: {}", p);
  println!("Vector3f of Point3f: {}", v_of_p);

  let transf = Translation3f::from(Vector3f::new(3.0, 3.0, 3.0));
  let new_p = transf * p;
  println!("New Point3f: {}", new_p);
  println!("New Vector3f: {}", Math::vector_of_point(&new_p));

  let new_v = Math::vector_of_point(&new_p);
  println!("Vector3f of new Point3f: {}", new_v);
}
