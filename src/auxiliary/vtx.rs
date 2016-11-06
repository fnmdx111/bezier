use na::{Point3, Vector3};


#[derive(Debug, Copy, Clone)]
pub struct Vertex {
  pos: [f64; 4],
  norm: [f64; 4]
}

implement_vertex!(Vertex, pos, norm);

impl Vertex {
  pub fn from_vector3(p: &Point3<f64>, n: &Vector3<f64>) -> Self {
    Vertex {
      pos: [p.x, p.y, p.z, 1.0],
      norm: [n.x, n.y, n.z, 0.0]
    }
  }
}
