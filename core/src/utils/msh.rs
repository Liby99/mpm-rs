use msh_rs::Node;
use super::Point3f;

pub fn msh_node_to_point(node: &Node) -> Point3f {
  Point3f::new(node.x as f32, node.y as f32, node.z as f32)
}