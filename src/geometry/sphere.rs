use crate::geometry::{Material, vec3::{Vec3}};

pub struct Sphere {
    pub(crate) center: Vec3,
    radius: f64,
    pub(crate) material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn ray_intersects(&self, orig: Vec3, dir: Vec3) -> (bool, f64) {
        let l = self.center - orig;
        let tca = l % dir;
        let d2 = (l % l) - (tca * tca);
        let r2 = self.radius * self.radius;

        if d2 > r2 {
            return (false, 0.0);
        }

        let thc = (r2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        if t0 > 0.001 {
            return (true, t0);
        }
        if t1 > 0.001 {
            return (true, t1);
        }

        (false, 0.0)
    }
}
