use std::f64::consts::PI;

pub mod color {
    use crate::geometry::{Material, vec3::Vec3};

    pub const BACKGROUND: Vec3 = Vec3::new(0.2, 0.7, 0.8);
    pub const BACKGROUND_M: Material = Material::new((1.0, 0.0, 0.0),BACKGROUND, 1.0);
}

pub const FOV: f64 = 1.05;
pub const SPHERES_MAX_DIST: f64 = 1000.0;
pub const SPERES_SHD: f64 = 0.001;
pub const REFLECTION_RECURSION_DEPTH: usize = 10;
