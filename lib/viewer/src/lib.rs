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
use na::{Point3, Translation3};

use mpm_rs::{
  components::{Hidden, ParticlePosition},
  Grid, Vector3f,
};

pub use ending::Ending;
use renderer::PointCloudRenderer;

type CamFxRdr<'a> = (
  Option<&'a mut dyn Camera>,
  Option<&'a mut dyn PlanarCamera>,
  Option<&'a mut dyn Renderer>,
  Option<&'a mut dyn PostProcessingEffect>,
);

pub struct WindowState {
  pub points: Vec<Point3<f32>>,
  pub renderer: PointCloudRenderer,
}

impl State for WindowState {
  fn cameras_and_effect_and_renderer(&mut self) -> CamFxRdr {
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

impl WindowSystem {
  pub fn new() -> Self {
    // Get window and ground plane
    let mut window = Window::new("MPM Viewer");
    let mut cube = window.add_cube(1.0, 0.01, 1.0);
    cube.set_local_translation(Translation3::new(0.0, -0.01, 0.0));
    cube.set_color(0.3, 0.3, 0.3);

    // Setup renderer and state
    let renderer = PointCloudRenderer::new();
    let state = WindowState {
      points: Vec::new(),
      renderer,
    };
    Self { window, state }
  }
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = (
    Write<'a, Ending>,
    Read<'a, Grid>,
    ReadStorage<'a, ParticlePosition>,
    ReadStorage<'a, Hidden>,
  );

  fn run(&mut self, (mut ending, grid, poses, hiddens): Self::SystemData) {
    // Store the offset
    let offset = Vector3f::new(-(grid.dim.x as f32), 0.0, -(grid.dim.z as f32)) * grid.h / 2.0;

    // First construct points
    let mut points = vec![];
    for (ParticlePosition(pos), _) in (&poses, !&hiddens).join() {
      let pos = pos + offset;
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
