pub mod sphere;
pub mod vec3;

trait Intersectable {
    fn ray_intersects(orig: &vec3::Vec3, dir: &vec3::Vec3, t0: f64) -> bool;
}