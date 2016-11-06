use na;
use na::{Matrix4, Point3, Vector3, PerspectiveMatrix3, Norm, ToHomogeneous};

use std::cmp::{max, min};
use std::default::Default;


pub struct Camera {
  r: f64,
  theta: f64,
  phi: f64,
  xmouse: i32,
  ymouse: i32,
  pub ctm: Matrix4<f64>,
  pub persp: Matrix4<f64>,
  eye: Point3<f64>
}

impl Default for Camera {
  fn default() -> Self {
    Camera {
      r: 5.0,
      theta: 1.0,
      phi: 1.0,
      xmouse: 0,
      ymouse: 0,
      persp: PerspectiveMatrix3::new(1.0, 40.0 / 180.0, 1.0, 50.0).to_matrix(),
      ctm: na::zero(),
      eye: na::origin()
    }
  }
}

impl Camera {
  pub fn refresh_mat(&mut self) {
    let y = self.r * self.phi.sin();
    let xz = self.r * self.phi.cos();
    let x = xz * self.theta.sin();
    let z = xz * self.theta.cos();

    self.eye.x = x;
    self.eye.y = y;
    self.eye.z = z;

    let view = -self.eye.as_vector();
    let side = na::cross(&view, &Vector3::new(0.0, 1.0, 0.0));
    let up = na::cross(&side, &view).normalize();

    self.ctm = na::Isometry3::look_at_rh(&self.eye,
                                         &Point3::new(0.0, 0.0, 0.0),
                                         &up).to_homogeneous();
  }

  pub fn rotate(&mut self, x: i32, y: i32) {
    let xdiff = max(-10, min(10, x - self.xmouse));
    let ydiff = max(-10, min(10, y - self.ymouse));

    if xdiff != 0 {
      self.theta += xdiff as f64;
      self.theta = if self.theta > 360.0 {
        self.theta - 360.0
      } else if self.theta < 0.0 {
        self.theta + 360.0
      } else {
        self.theta
      };

      self.xmouse = x;
    }

    if ydiff != 0 {
      self.phi += ydiff as f64;
      self.phi = <f64>::min(265.0, <f64>::max(95.0, self.phi));

      self.ymouse = y;
    }
  }

  pub fn reset(&mut self) {
    self.r = 5.0;
    self.theta = 180.0;
    self.phi = 180.0;
  }

  pub fn zoom_in(&mut self) {
    self.r = <f64>::max(2.0, self.r - 1.0);
  }

  pub fn zoom_out(&mut self) {
    self.r = <f64>::min(50.0, self.r + 1.0);
  }
}
