use rand::Rng;

use super::mpm::*;
use super::math::*;

pub fn sample_point_in_sphere(center: Vector3f, radius: f32) -> Vector3f {
  let mut rng = rand::thread_rng();
  loop {
    let x = rng.gen_range(-radius, radius);
    let y = rng.gen_range(-radius, radius);
    let z = rng.gen_range(-radius, radius);
    let v = Vector3f::new(x, y, z);
    if v.magnitude() <= radius {
      return center + v;
    }
  }
}

pub fn put_zero_boundary(world: &mut World, thickness: f32) {
  let grid = &mut world.grid;
  let num_nodes = (thickness / grid.h).ceil() as usize;
  for node in &mut grid.nodes {
    let boundary = if node.index.x < num_nodes {
      Some(Boundary::SetZero)
    } else if node.index.x > grid.dim.x - num_nodes {
      Some(Boundary::SetZero)
    } else if node.index.y < num_nodes {
      Some(Boundary::SetZero)
    } else if node.index.y > grid.dim.y - num_nodes {
      Some(Boundary::SetZero)
    } else if node.index.z < num_nodes {
      Some(Boundary::SetZero)
    } else if node.index.z > grid.dim.z - num_nodes {
      Some(Boundary::SetZero)
    } else {
      None
    };
    node.boundary = boundary;
  }
}

pub fn put_boundary(world: &mut World, thickness: f32) {
  let grid = &mut world.grid;
  let num_nodes = (thickness / grid.h).ceil() as usize;
  for node in &mut grid.nodes {
    let boundary = if node.index.x < num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(1.0, 0.0, 0.0) })
    } else if node.index.x > grid.dim.x - num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(-1.0, 0.0, 0.0) })
    } else if node.index.y < num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(0.0, 1.0, 0.0) })
    } else if node.index.y > grid.dim.y - num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(0.0, -1.0, 0.0) })
    } else if node.index.z < num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(0.0, 0.0, 1.0) })
    } else if node.index.z > grid.dim.z - num_nodes {
      Some(Boundary::Surface { normal: Vector3f::new(0.0, 0.0, -1.0) })
    } else {
      None
    };
    node.boundary = boundary;
  }
}

pub fn put_ball(
  world: &mut World,
  center: Vector3f,
  radius: f32,
  num_particles: usize,
  total_mass: f32,
) {
  let ind_mass = total_mass / (num_particles as f32);
  for _ in 0..num_particles {
    let pos = sample_point_in_sphere(center, radius);
    let par = Particle::new(ind_mass, pos);
    world.particles.push(par);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gen_world_1() {
    let mut world = World::new(Vector3f::new(1.0, 1.0, 1.0), 0.02);
    put_boundary(&mut world, 0.03);
    put_ball(&mut world, Vector3f::new(0.5, 0.8, 0.5), 0.1, 1000, 1.0);
    assert_eq!(world.particles.len(), 1000);
  }
}