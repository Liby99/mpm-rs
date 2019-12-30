use kiss3d::resource::{Effect, ShaderAttribute, ShaderUniform, GPUVec,
  BufferType, AllocationType};
use kiss3d::context::Context;
use kiss3d::renderer::Renderer;
use kiss3d::camera::Camera;
use na::{Point3, Matrix4};

pub struct PointCloudRenderer {
  shader: Effect,
  pos: ShaderAttribute<Point3<f32>>,
  color: ShaderAttribute<Point3<f32>>,
  proj: ShaderUniform<Matrix4<f32>>,
  view: ShaderUniform<Matrix4<f32>>,
  colored_points: GPUVec<Point3<f32>>,
  point_size: f32,
}

impl PointCloudRenderer {
  pub fn new(point_size: f32) -> PointCloudRenderer {
    let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
    shader.use_program();
    PointCloudRenderer {
      colored_points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
      pos: shader.get_attrib::<Point3<f32>>("position").unwrap(),
      color: shader.get_attrib::<Point3<f32>>("color").unwrap(),
      proj: shader.get_uniform::<Matrix4<f32>>("proj").unwrap(),
      view: shader.get_uniform::<Matrix4<f32>>("view").unwrap(),
      shader,
      point_size,
    }
  }

  pub fn set(&mut self, points: &Vec<Point3<f32>>) {
    if let Some(colored_points) = self.colored_points.data_mut() {
      colored_points.clear();
      for p in points {
        colored_points.push(Point3::new(p.x, p.y, p.z));
        colored_points.push(Point3::new(1.0, 1.0, 1.0)); // White color
      }
    }
  }
}

impl Renderer for PointCloudRenderer {
  fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
    if self.colored_points.len() == 0 {
      return;
    }

    self.shader.use_program();
    self.pos.enable();
    self.color.enable();

    camera.upload(pass, &mut self.proj, &mut self.view);

    self.color.bind_sub_buffer(&mut self.colored_points, 1, 1);
    self.pos.bind_sub_buffer(&mut self.colored_points, 1, 0);

    let ctxt = Context::get();
    ctxt.point_size(self.point_size);
    ctxt.draw_arrays(Context::POINTS, 0, (self.colored_points.len() / 2) as i32);

    self.pos.disable();
    self.color.disable();
  }
}

const VERTEX_SHADER_SRC: &'static str = "
  #version 100
  attribute vec3 position;
  attribute vec3 color;
  varying   vec3 Color;
  uniform   mat4 proj;
  uniform   mat4 view;
  void main() {
    gl_Position = proj * view * vec4(position, 1.0);
    Color = color;
  }";

const FRAGMENT_SHADER_SRC: &'static str = "
  #version 100
  #ifdef GL_FRAGMENT_PRECISION_HIGH
    precision highp float;
  #else
    precision mediump float;
  #endif
  varying vec3 Color;
  void main() {
    gl_FragColor = vec4(Color, 1.0);
  }";
