extern crate nalgebra as na;
#[macro_use]
extern crate gfx;
extern crate gfx_app;

mod bezier;
mod auxiliary;
mod scene;

use bezier::traits::*;
use bezier::BezierObject;
use scene::{Camera, Scene, Light, Material};
use auxiliary::Vertex;

use std::default::Default;
use std::convert::From;

pub use gfx_app::{ColorFormat, DepthFormat};
use gfx::{Bundle};

gfx_defines! {
  vertex GfxVertex {
    pos: [f32; 4] = "a_position",
    norm: [f32; 4] = "a_norm",
  }

  constant LightObject {
    ambient: [f32; 4] = "ambient",
    diffuse: [f32; 4] = "diffuse",
    specular: [f32; 4] = "specular",
    pos: [f32; 4] = "pos",
  }

  constant MaterialObject {
    ambient: [f32; 4] = "ambient",
    diffuse: [f32; 4] = "diffuse",
    specular: [f32; 4] = "specular",
    shininess: f32 = "shininess",
  }

  pipeline bzr {
    vbuf: gfx::VertexBuffer<GfxVertex> = (),
    ctm: gfx::Global<[[f32; 4]; 4]> = "u_ctm",
    persp: gfx::Global<[[f32; 4]; 4]> = "u_persp",
    eye: gfx::Global<[f32; 4]> = "u_eye",
    light: gfx::ConstantBuffer<LightObject> = "u_light",
    material: gfx::ConstantBuffer<MaterialObject> = "u_material",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}

macro_rules! unpack {
  (pos $i:expr) => ([$i.x as f32, $i.y as f32, $i.z as f32, 1.0f32]);
  (vec $i:expr) => ([$i.x as f32, $i.y as f32, $i.z as f32, 0.0f32]);
  (v4 $i:expr) => ([$i.x as f32, $i.y as f32, $i.z as f32, $i.w as f32]);
}

impl From<Light> for LightObject {
  fn from(l: Light) -> Self {
    LightObject {
      ambient: unpack!(v4 l.ambient),
      diffuse: unpack!(v4 l.diffuse),
      specular: unpack!(v4 l.specular),
      pos: unpack!(pos l.pos)
    }
  }
}

impl From<Material> for MaterialObject {
  fn from(m: Material) -> Self {
    MaterialObject {
      ambient: unpack!(v4 m.ambient),
      diffuse: unpack!(v4 m.diffuse),
      specular: unpack!(v4 m.specular),
      shininess: m.shininess
    }
  }
}

impl From<Vertex> for GfxVertex {
  fn from(v: Vertex) -> Self {
    GfxVertex {
      pos: v.pos.clone(),
      norm: v.norm.clone()
    }
  }
}


struct App<R: gfx::Resources> {
  bundle: Bundle<R, bzr::Data<R>>,
  scene: Scene
}

use gfx::Resources;

impl<R> gfx_app::Application<R> for App<R> where R: gfx::Resources {
  fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
    use gfx::traits::FactoryExt;

    let vs = gfx_app::shade::Source {
      glsl_150: include_bytes!("vshader_passthrough.glsl"),
      .. gfx_app::shade::Source::empty()
    };
    let ps = gfx_app::shade::Source {
      glsl_150: include_bytes!("fshader_passthrough.glsl"),
      .. gfx_app::shade::Source::empty()
    };

    let mut bz = BezierObject::new(1);
    bz.read_bezier_file("fandisk.txt");

    let mut scene = Scene::new(Camera::default(), bz, Light::default(), Material::default());
    let vtxs = scene.bobj.refresh();
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vtxs, ());
    let pso = factory.create_pipeline_simple(vs.select(init.backend).unwrap(),
                                             ps.select(init.backend).unwrap(),
                                             bzr::new()).unwrap();

    use na::ToHomogeneous;

    let data = bzr::Data {
      vbuf: vbuf,
      ctm: scene.camera.ctm.as_ref(),
      persp: scene.camera.persp.as_ref(),
      eye: scene.camera.eye.to_homogeneous().as_ref(),
      light: LightObject::from(scene.light),
      material: MaterialObject::from(scene.material),
      out_color: init.color,
      out_depth: init.depth
    };

    App {
      bundle: Bundle::new(slice, pso, data),
      scene: scene
    }
  }

  fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
    self.scene.camera.refresh_mat();
    encoder.update_constant_buffer(&self.bundle.data.ctm,
                                   self.scene.camera.ctm.as_ref());

    encoder.update_buffer(&self.bundle.data.vbuf,
                          &self.scene.bobj.refresh());

    encoder.clear(&self.bundle.data.out_color, [1.0, 1.0, 1.0, 1.0]);
    encoder.clear(&self.bundle.data.out_depth, 1.0);

    self.bundle.encode(encoder);
  }
}

fn main() {
  use gfx_app::Application;
  App::launch_default("Bezier");
}
