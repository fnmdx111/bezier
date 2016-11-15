
use na::Vector4;

use std::default::Default;


pub struct Material {
    pub ambient: Vector4<f64>,
    pub diffuse: Vector4<f64>,
    pub specular: Vector4<f64>,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: Vector4::new(1.0, 0.0, 1.0, 1.0),
            diffuse: Vector4::new(1.0, 0.8, 0.0, 1.0),
            specular: Vector4::new(1.0, 0.8, 0.0, 1.0),
            shininess: 100.0,
        }
    }
}
