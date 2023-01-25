use crate::geometry::vec3::{Vec3, ZERO_VEC};

pub mod light;
pub mod sphere;
pub mod vec3;

pub trait Intersectable {
    fn ray_intersects(orig: &Vec3, dir: &Vec3, t0: f64) -> bool;
}

#[derive(Copy, Clone)]
pub struct Material {
    pub(crate) albedo: (f64, f64, f64, f64),
    pub(crate) diffuse_color: Vec3,
    pub(crate) specular_exponent: f64,
    pub(crate) refractive_index: f64,
}

impl Material {
    pub const fn new(
        albedo: (f64, f64, f64, f64),
        diffuse_color: Vec3,
        specular_exponent: f64,
        refractive_index: f64,
    ) -> Material {
        Material {
            albedo,
            diffuse_color,
            specular_exponent,
            refractive_index,
        }
    }

    pub fn new_blank() -> Material {
        Material {
            albedo: (1.0, 0.0, 0.0, 0.0),
            diffuse_color: ZERO_VEC,
            specular_exponent: 0.0,
            refractive_index: 0.0,
        }
    }
}
