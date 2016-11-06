use na;
use na::{Vector3, Point3};
use super::bezier_curve::BezierCurve;

use auxiliary::Vertex;

use super::{Eval, Tessellate, CountVertices};


#[derive(Debug)]
pub struct BezierPatch {
  deg_u: usize,
  deg_v: usize,
  vtxs: Vec<Vec<Point3<f64>>>,
  tgns: Vec<Vec<Vector3<f64>>>,
  ubzrs: Vec<BezierCurve>,
  vbzrs: Vec<BezierCurve>
}

impl BezierPatch {
  pub fn new(cs: &Vec<Vec<Point3<f64>>>) -> Self {
    let deg_u = cs[0].len() - 1;
    let deg_v = cs.len() - 1;

    let mut bzp = BezierPatch {
      deg_u: deg_u,
      deg_v: deg_v,
      vtxs: vec![],
      tgns: vec![],
      ubzrs: Vec::with_capacity(deg_v + 1),
      vbzrs: Vec::with_capacity(deg_u + 1)
    };

    for v in 0..(deg_v + 1) {
      bzp.ubzrs.push(BezierCurve::new(&cs[v]));
    }

    for u in 0..(deg_u + 1) {
      let mut cv = Vec::with_capacity(deg_v + 1);
      for v in 0..(deg_v + 1) {
        cv.push(cs[v][u]);
      }

      bzp.vbzrs.push(BezierCurve::new(&cv));
    }

    bzp
  }

  fn resize_vectors(&mut self, n_sample: usize) {
    self.vtxs.clear();
    self.tgns.clear();

    let n_vtx_v = n_sample * self.deg_v;
    let n_vtx_u = n_sample * self.deg_u;
    self.vtxs.resize(n_vtx_v, vec![na::origin(); n_vtx_u]);
    self.tgns.resize(n_vtx_v, vec![na::zero(); n_vtx_u]);
  }
}

impl Eval for BezierPatch {
  fn eval(&mut self, n_sample: usize) {
    let ndeg_u = n_sample * self.deg_u;
    let ndeg_v = n_sample * self.deg_v;

    self.resize_vectors(n_sample);

    for v in 0..(self.deg_v + 1) {
      self.ubzrs[v].eval(n_sample);
    }

    let mut nvbzrs = Vec::with_capacity(ndeg_u);
    for u in 0..ndeg_u {
      let v_points = self.ubzrs.iter()
        .map(|bz| bz.vertices[u])
        .collect::<Vec<Point3<f64>>>();

      let mut bz = BezierCurve::new(&v_points);
      bz.eval(n_sample);
      nvbzrs.push(bz);
    }

    for u in 0..ndeg_u {
      for v in 0..ndeg_v {
        self.tgns[v][u] = nvbzrs[u].tangents[v];
        self.vtxs[v][u] = nvbzrs[u].vertices[v];
      }
    }

    for u in 0..(self.deg_u + 1) {
      self.vbzrs[u].eval(n_sample);
    }

    let mut nubzrs = Vec::with_capacity(ndeg_v);
    for v in 0..ndeg_v {
      let u_points = self.vbzrs.iter()
        .map(|bz| bz.vertices[v])
        .collect::<Vec<Point3<f64>>>();

      let mut bz = BezierCurve::new(&u_points);
      bz.eval(n_sample);
      nubzrs.push(bz);
    }

    for v in 0..ndeg_v {
      for u in 0..ndeg_u {
        let v_tgn = self.tgns[v][u];
        let u_tgn = nubzrs[v].tangents[u];
        self.tgns[v][u] = na::cross(&v_tgn, &u_tgn);
      }
    }
  }
}

impl Tessellate for BezierPatch {
  fn tess(&self) -> Vec<Vertex> {
    let ndeg_u = self.vtxs[0].len() - 1;
    let ndeg_v = self.vtxs.len() - 1;
    let mut ret = Vec::<Vertex>::new();

    macro_rules! put {
      ($r:ident; $( ( $v:expr, $u:expr ) ),+) =>
        ($(
          $r.push(Vertex::from_vector3(&self.vtxs[$v][$u], &self.tgns[$v][$u])));+)
    }

    for v in 0..ndeg_v {
      for u in 0..ndeg_u {
        put!(ret;
          (v + 1, u), (v, u + 1), (v, u),
          (v + 1, u), (v, u + 1), (v + 1, u + 1));
      }
    }

    ret
  }
}

impl CountVertices for BezierPatch {
  fn n_vertices(&self) -> usize {
    let ndeg_u = self.vtxs[0].len() - 1;
    let ndeg_v = self.vtxs.len() - 1;

    ndeg_u * ndeg_v * 2 * 3
  }
}
