extern crate nalgebra as na;

use mpm_rs::*;
use na::*;

#[test]
fn bounding_box_transf_1() {
  let min = Point3f::new(0.0, 0.0, 0.0);
  let max = Point3f::new(1.0, 1.0, 1.0);
  let bb = BoundingBox::new(min, max);
  let transf = Translation3f::from(Vector3f::new(3.0, 3.0, 3.0));
  let new_bb = bb.transform(&na::convert(transf));
  assert_eq!(new_bb.min, Point3f::new(3.0, 3.0, 3.0));
  assert_eq!(new_bb.max, Point3f::new(4.0, 4.0, 4.0));
}

#[test]
fn bounding_box_transf_2() {
  let min = Point3f::new(0.0, 0.0, 0.0);
  let max = Point3f::new(1.0, 1.0, 1.0);
  let bb = BoundingBox::new(min, max);
  let transl = Translation3f::from(Vector3f::new(3.0, 3.0, 3.0));
  let rotate = UnitQuaternion::from_axis_angle(
    &Unit::new_normalize(Vector3f::new(0.0, 1.0, 0.0)),
    std::f32::consts::FRAC_PI_2,
  );
  let transf = Isometry3f::from_parts(transl, rotate);
  let new_bb = bb.transform(&na::convert(transf));
  assert_eq!(new_bb.min, Point3f::new(3.0, 3.0, 2.0));
  assert_eq!(new_bb.max, Point3f::new(4.0, 4.0, 3.0));
}
