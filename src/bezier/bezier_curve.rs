use na::{Point3, Vector3};
use na;

use super::Eval;


#[derive(Debug)]
pub struct BezierCurve {
  deg: usize,
  ctrls: Vec<Point3<f64>>,
  pub vertices: Vec<Point3<f64>>,
  pub tangents: Vec<Vector3<f64>>
}

impl BezierCurve {
  pub fn new(ctrls: &Vec<Point3<f64>>) -> BezierCurve {
    BezierCurve {
      deg: ctrls.len() - 1,
      ctrls: ctrls.clone(),
      vertices: vec![],
      tangents: vec![]
    }
  }

  fn de_casteljau_sub(&self, t: f64) -> (Point3<f64>, Vector3<f64>) {
    let mut dc_ctrls = self.ctrls.clone();
    let deg_p_1 = self.deg + 1;

    let mut tangent = na::zero();

    for i in 1..deg_p_1 {
      if i == self.deg {
        tangent = (self.deg as f64) * (dc_ctrls[self.deg] - dc_ctrls[self.deg - 1])
      }

      for j in (i..(self.deg + 1)).rev() {
        dc_ctrls[j] = ((1.0 - t) * dc_ctrls[j - 1].as_vector()
          + t * dc_ctrls[j].as_vector()).to_point();
      }
    };

    (dc_ctrls[self.deg], tangent)
  }
}

impl Eval for BezierCurve {
  fn eval(&mut self, n_sample: usize) {
    let n_vtx = n_sample * self.deg;

    self.vertices.clear();
    self.tangents.clear();
    self.vertices.resize(n_vtx, na::origin());
    self.tangents.resize(n_vtx, na::zero());

    let t_step = 1.0 / (n_vtx as f64 - 1.0);
    let mut t = 0.0;
    for i in 0..self.deg {
      for s in 0..n_sample {
        let n = i * n_sample + s;
        let (vtx, tgn) = self.de_casteljau_sub(t);
        self.vertices[n] = vtx;
        self.tangents[n] = tgn;

        t += t_step;
      }
    };
  }
}
