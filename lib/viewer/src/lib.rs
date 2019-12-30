extern crate kiss3d;
extern crate mpm_rs;
extern crate nalgebra as na;
extern crate specs;

mod ending;
mod renderer;

use specs::prelude::*;

use kiss3d::camera::Camera;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::window::{State, Window};
use na::Point3;

use mpm_rs::components::ParticlePosition;

use renderer::PointCloudRenderer;
pub use ending::Ending;

pub struct WindowState {
  pub points: Vec<Point3<f32>>,
  pub renderer: PointCloudRenderer,
}

impl State for WindowState {
  fn cameras_and_effect_and_renderer(&mut self) -> (
    Option<&mut dyn Camera>,
    Option<&mut dyn PlanarCamera>,
    Option<&mut dyn Renderer>,
    Option<&mut dyn PostProcessingEffect>,
  ) {
    (None, None, Some(&mut self.renderer), None)
  }

  fn step(&mut self, _: &mut Window) {
    self.renderer.set(&self.points);
  }
}

pub struct WindowSystem {
  window: Window,
  state: WindowState,
}

impl Default for WindowSystem {
  fn default() -> Self {
    let window = Window::new("MPM Viewer");
    let renderer = PointCloudRenderer::new(4.0);
    let state = WindowState { points: Vec::new(), renderer };
    Self { window, state }
  }
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Write<'a, Ending>,
    ReadStorage<'a, ParticlePosition>
  );

  fn run(&mut self, (mut ending, poses): Self::SystemData) {

    // First construct points
    let mut points = vec![];
    for ParticlePosition(pos) in (&poses).join() {
      points.push(Point3::new(pos.x, pos.y, pos.z));
    }

    // Then update points
    self.state.points = points;

    // Finally render the window
    if !self.window.render_with_state(&mut self.state) {
      ending.set_ended();
    }
  }
}
