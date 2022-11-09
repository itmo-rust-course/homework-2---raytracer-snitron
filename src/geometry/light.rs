use crate::geometry::vec3::Vec3;

pub struct Light {
    pub(crate) position: Vec3,
    pub(crate) intensity: f64
}

impl Light {
    pub fn new(position: Vec3, intensity: f64) -> Light {
       Light { position, intensity }
    }
}