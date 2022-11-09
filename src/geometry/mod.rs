use crate::geometry::vec3::{Vec3, ZERO_VEC};

pub mod sphere;
pub mod vec3;
pub mod light;

trait Intersectable {
    fn ray_intersects(orig: &vec3::Vec3, dir: &vec3::Vec3, t0: f64) -> bool;
}

#[derive(Copy, Clone)]
pub struct Material {
    albedo: (f64, f64),
    diffuse_color: Vec3,
    specular_exponent: f64
}

impl Material {
    pub const fn new(albedo: (f64, f64), diffuse_color: Vec3, specular_exponent: f64) -> Material {
        Material { albedo, diffuse_color, specular_exponent }
    }

    fn new_blank() -> Material {
        Material { albedo: (1.0, 0.0), diffuse_color: ZERO_VEC, specular_exponent: 0.0 }
    }
}