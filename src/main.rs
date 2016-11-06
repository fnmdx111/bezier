extern crate nalgebra as na;
#[macro_use] extern crate glium;

mod bezier;
mod auxiliary;
mod scene;

use bezier::traits::*;
use bezier::BezierObject;
use scene::{Camera, Scene, Light, Material};

use std::default::Default;


fn gl_init(scene: &Scene) {
  use glium::DisplayBuild;
  let display = glium::glutin::WindowBuilder::new()
    .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGl, (4, 1)))
    .with_dimensions(1024, 768)
    .with_title("Hello world")
    .build_glium()
    .unwrap();

  let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &scene.bobj.tess());
  let program = glium::Program::from_source(&display,
                                            "vshader_passthrough.glsl",
                                            "fshader_passthrough.glsl",
                                            None);
  match program {
    Ok(program) => {
      let index = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    },
    Err(err) => {
      println!("{}", err);
    }
  }
}


fn main() {
  let mut bz = BezierObject::new(1);
  bz.read_bezier_file("fandisk.txt");

  let mut scene = Scene::new(
    Camera::default(), bz, Light::default(), Material::default()
  );

  gl_init(&scene);
}
