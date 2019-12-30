extern crate kiss3d;
extern crate mpm_rs;
extern crate nalgebra as na;
extern crate specs;

mod renderer;

use std::sync::mpsc;
use std::thread;

use specs::prelude::*;

use kiss3d::camera::Camera;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::window::{State, Window};

use mpm_rs::components::ParticlePosition;
use mpm_rs::Vector3f;

use renderer::PointCloudRenderer;

struct Message(pub Vec<Vector3f>);

struct AppState {
  renderer: PointCloudRenderer,
  receiver: mpsc::Receiver<Message>,
}

impl State for AppState {
  fn cameras_and_effect_and_renderer(
    &mut self,
  ) -> (
    Option<&mut dyn Camera>,
    Option<&mut dyn PlanarCamera>,
    Option<&mut dyn Renderer>,
    Option<&mut dyn PostProcessingEffect>,
  ) {
    (None, None, Some(&mut self.renderer), None)
  }

  fn step(&mut self, _: &mut Window) {
    match self.receiver.try_recv() {
      Ok(Message(points)) => {
        self.renderer.set(points)
      },
      _ => (),
    }
  }
}

pub struct WindowSystem {
  sender: mpsc::Sender<Message>,
}

impl Default for WindowSystem {
  fn default() -> Self {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
      let window = Window::new("MPM Viewer");
      let app = AppState {
        renderer: PointCloudRenderer::new(4.0),
        receiver: receiver,
      };
      window.render_loop(app);
    });

    Self { sender }
  }
}

impl<'a> System<'a> for WindowSystem {
  type SystemData = ReadStorage<'a, ParticlePosition>;

  fn run(&mut self, poses: Self::SystemData) {
    let mut ps = vec![];
    for ParticlePosition(pos) in (&poses).join() {
      ps.push(pos.clone());
    }
    let _ = self.sender.send(Message(ps));
  }
}
