
use na;
use na::{Vector4, Point3};

use std::default::Default;


pub struct Light {
    pub pos: Point3<f64>,
    pub ambient: Vector4<f64>,
    pub diffuse: Vector4<f64>,
    pub specular: Vector4<f64>,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            pos: Point3::new(100.0, 100.0, 100.0),
            ambient: Vector4::new(0.2, 0.2, 0.2, 1.0),
            diffuse: Vector4::new(1.0, 1.0, 1.0, 1.0),
            specular: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
