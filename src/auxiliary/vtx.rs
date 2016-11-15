use na::{Point3, Vector3};

pub struct Vertex {
  pub pos: [f32; 4],
  pub norm: [f32; 4]
}

impl Vertex {
  pub fn from_vector3(p: &Point3<f64>, n: &Vector3<f64>) -> Self {
    Vertex {
      pos: [p.x as f32, p.y as f32, p.z as f32, 1.0],
      norm: [n.x as f32, n.y as f32, n.z as f32, 0.0],
    }
  }
}

