use std::f64::consts::PI;

pub mod colour {
    use crate::geometry::vec3::Vec3;

    pub const BACKGROUND: Vec3 = Vec3::new(0.2, 0.7, 0.8);
}

pub const FOV: f64 = PI / 2.0;
