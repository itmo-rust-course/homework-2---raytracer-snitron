use std::thread::spawn;

use crate::geometry::{Intersectable, vec3::{Vec3, ZERO_VEC}};
use crate::render::{constants::{colour::BACKGROUND, FOV}, ImageMatrix, Renderable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub(crate) fn ray_intersects(&self, orig: &Vec3, dir: &Vec3, t0: f64) -> (bool, f64) {
        let l = self.center - *orig;
        let tca = l % (*dir);
        let d2 = (l % l) - (tca * tca);
        let r2 = self.radius * self.radius;

        if d2 > r2 {
            return (false, t0);
        }

        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            println!("CUM2 {}", t0);

            return (false, t0);
        }

        return (true, t0);
    }

    pub fn cast_ray(&self, orig: &Vec3, dir: &Vec3) -> Vec3 {
        let (intersects, sphere_dist) = self.ray_intersects(orig, dir, f64::MAX);
        if !intersects {
            return BACKGROUND; // background color
        }

        return Vec3::new(0.4, 0.4, 0.3);
    }
}

impl Renderable for Sphere {
    fn render(&self, framebuffer: &mut ImageMatrix) {
        let (w, h) = framebuffer.get_dimensions();
        let (wf, hf) = (w as f64, h as f64);
        let fov_2tan = (FOV / 2.0).tan();

        let dir = Vec3::new(-0.182498, -0.122822, -0.975505);

        //println!("{}", self.ray_intersects(&ZERO_VEC, &dir, 0.0).0)
        for j in 0..h {
            for i in 0..w {
                let x = (2.0 * (i as f64 + 0.5) / wf - 1.0) * fov_2tan * wf / hf;
                let y = -(2.0 * (j as f64 + 0.5) / hf - 1.0) * fov_2tan;

                let dir = Vec3::new(x, y, -1.0).normalize();
                if dir.x < 0.0 {
                    // println!("{}", dir);
                }


                framebuffer.put_pixel(i, j, &self.cast_ray(&ZERO_VEC, &dir))
            }
        }
    }
}