extern crate specs;
extern crate nalgebra as na;
extern crate kiss3d;

pub struct WindowSystem {
  pub window: kiss3d::window::Window,
}

impl WindowSystem {
  pub fn new(name: &str) -> Self {
    Self { window: kiss3d::window::Window::new(name) }
  }
}