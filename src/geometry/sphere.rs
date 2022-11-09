use crate::geometry::{Intersectable, Material, vec3::{Vec3, ZERO_VEC}};
use crate::geometry::light::Light;
use crate::geometry::vec3::ONE_VEC;
use crate::render::{constants::{color::BACKGROUND, FOV}, ImageMatrix};
use crate::render::constants::color::BACKGROUND_M;
use crate::render::constants::SPHERES_MAX_DIST;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub(crate) fn ray_intersects(&self, orig: Vec3, dir: Vec3, t0: f64) -> (bool, f64) {
        let l = self.center - orig;
        let tca = l % dir;
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
            return (false, t0);
        }

        return (true, t0);
    }


}

pub struct RenderUtils;

impl RenderUtils {
    fn reflect(i: Vec3, n: Vec3) -> Vec3 {
        i - n.mul_const(2.0) * (i * n)
    }

    pub fn render_spheres(
        framebuffer: &mut ImageMatrix,
        spheres: &Vec<Sphere>,
        lights: &Vec<Light>
    ) {
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


                framebuffer.put_pixel(i, j, RenderUtils::cast_ray(ZERO_VEC, dir, spheres, lights))
            }
        }
    }

    pub fn spheres_intercects(
        orig: Vec3,
        dir: Vec3,
        hit_point: &mut Vec3,
        n: &mut Vec3,
        spheres: &Vec<Sphere>
    ) -> (bool, Material) {
        let mut spheres_dist = f64::MAX;
        let mut material = BACKGROUND_M;

        for sphere in spheres {
            let (intersects, dist) = sphere.ray_intersects(orig, dir, spheres_dist);
            if intersects && dist < spheres_dist {
                spheres_dist = dist;
                *hit_point = orig + dir.mul_const(dist);
                *n = (*hit_point - sphere.center).normalize();
                material = sphere.material;
            }
        }

        return  (spheres_dist < SPHERES_MAX_DIST, material);
    }

    pub fn cast_ray(
        orig: Vec3,
        dir: Vec3,
        spheres: &Vec<Sphere>,
        lights: &Vec<Light>
    ) -> Vec3 {
        let mut hit_point = ZERO_VEC;
        let mut n = ZERO_VEC;

        let (intersects, material) = RenderUtils::spheres_intercects(orig, dir, &mut hit_point, &mut n, spheres);

        if !intersects {
            return BACKGROUND;
        }

        let mut diffuse_intensity = 0.0;
        let mut specular_intensity = 0.0;

        for light in lights {
            let light_dir = (light.position - hit_point).normalize();
            diffuse_intensity += 0.0_f64.max(light_dir % n) * light.intensity;
            specular_intensity += (0.0_f64.max(RenderUtils::reflect(light_dir, n) % dir))
                .powf(material.specular_exponent) * light.intensity;
        }

        return material.diffuse_color
            .mul_const(diffuse_intensity)
            .mul_const(material.albedo.0)
            + ONE_VEC.mul_const(specular_intensity)
            .mul_const(material.albedo.1);
    }
}