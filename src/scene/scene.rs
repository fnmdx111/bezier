
use scene::Camera;
use scene::Light;
use scene::Material;

use bezier::BezierObject;
use bezier::EvalS;


pub struct Scene {
  pub camera: Camera,
  pub bobj: BezierObject,
  pub light: Light,
  pub material: Material
}

impl Scene {
  pub fn new(mut camera: Camera,
             mut bobj: BezierObject,
             light: Light,
             material: Material) -> Self {
    camera.refresh_mat();
    bobj.eval();

    Scene {
      camera: camera,
      bobj: bobj,
      light: light,
      material: material
    }
  }

  pub fn reload(&mut self) {
    self.bobj.refresh();
  }
}
