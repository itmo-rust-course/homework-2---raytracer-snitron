use crate::geometry::vec3::{Vec3, ZERO_VEC};

pub mod sphere;
pub mod vec3;
pub mod light;

trait Intersectable {
    fn ray_intersects(orig: &vec3::Vec3, dir: &vec3::Vec3, t0: f64) -> bool;
}

#[derive(Copy, Clone)]
pub struct Material {
    diffuse_color: Vec3
}

impl Material {
    pub const fn new(diffuse_color: Vec3) -> Material {
        Material { diffuse_color }
    }

    fn new_blank() -> Material {
        Material { diffuse_color: ZERO_VEC }
    }
}