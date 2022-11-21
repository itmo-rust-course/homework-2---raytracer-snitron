use image::{ImageResult, RgbImage};
use crate::geometry::{vec3::Vec3};
use crate::geometry::light::Light;
use crate::geometry::sphere::Sphere;
use crate::geometry::vec3::ZERO_VEC;
use crate::render::constants::FOV;

pub mod constants;

pub struct ImageMatrix {
    framebuffer: RgbImage
}

impl ImageMatrix {
    pub fn new(w: u32, h: u32) -> ImageMatrix {
        ImageMatrix { framebuffer: RgbImage::new(w, h) }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.framebuffer.dimensions()
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, vec: Vec3) {
        self.framebuffer.put_pixel(x, y, vec.into());
    }

    pub fn save_image(self, path: &str) -> ImageResult<()> {
        self.framebuffer.save(path)
    }
}

mod render_impl {
    use crate::geometry::{Material, vec3::{Vec3, ZERO_VEC}};
    use crate::geometry::light::Light;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::vec3::ONE_VEC;
    use crate::render::constants::color::{BACKGROUND, BACKGROUND_M};
    use crate::render::constants::{REFLECTION_RECURSION_DEPTH, SPHERES_SHD, SPHERES_MAX_DIST};

    fn reflect(i: Vec3, n: Vec3) -> Vec3 {
        i - n.mul_const(2.0 * (i % n))
    }

    fn refract(i: Vec3, n: Vec3, eta_t: f64, eta_i: f64) -> Vec3 {
        let cos_i = -(-1.0_f64).max(1.0_f64.min(i % n));
        if cos_i < 0.0 { return refract(i, -n, eta_t, eta_t); }

        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);

        if k < 0.0 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            i.mul_const(eta) + n.mul_const(eta * cos_i - k.sqrt())
        }
    }


    fn spheres_intercects(
        orig: Vec3,
        dir: Vec3,
        hit_point: &mut Vec3,
        n: &mut Vec3,
        spheres: &Vec<Sphere>
    ) -> (bool, Material) {
        let mut spheres_dist = f64::MAX;
        let mut material = BACKGROUND_M;

        for sphere in spheres {
            let (intersects, dist) = sphere.ray_intersects(orig, dir);
            if intersects && dist < spheres_dist {
                spheres_dist = dist;
                *hit_point = orig + dir.mul_const(dist);
                *n = (*hit_point - sphere.center).normalize();
                material = sphere.material;
            }
        }

        let mut checkerboard_dist = f64::MAX;

        if dir.y.abs() > 1e-3 {
            let d = -(orig.y + 4.0) / dir.y;
            let pt = orig + dir.mul_const(d);

            if d > 0.0 && pt.x.abs() < 10.0 && pt.z > -30.0 && d < spheres_dist {
                checkerboard_dist = d;
                *hit_point = pt;
                *n = Vec3::new(0.0, 1.0, 0.0);
                material.diffuse_color = if (((0.5 * hit_point.x + 1000.0) as i64) + (0.5 * hit_point.z) as i64) & 1_i64 == 1 { ONE_VEC } else { Vec3::new(1.0, 0.7, 0.3) }
            }
        }
        (spheres_dist.min(checkerboard_dist) < SPHERES_MAX_DIST, material)
    }

    pub(crate) fn cast_ray(
    orig: Vec3,
    dir: Vec3,
    spheres: &Vec<Sphere>,
    lights: &Vec<Light>,
    depth: usize
) -> Vec3 {
    let mut hit_point = ZERO_VEC;
    let mut n = ZERO_VEC;

    if depth > REFLECTION_RECURSION_DEPTH {
        return BACKGROUND;
    }

    let (intersects, material) = spheres_intercects(orig, dir, &mut hit_point, &mut n, spheres);

    if !intersects {
        return BACKGROUND;
    }

    let reflect_dir = reflect(dir, n).normalize();
    let refract_dir = refract(dir, n, material.refractive_index, 1.0).normalize();

    let reflect_color = cast_ray(hit_point, reflect_dir, spheres, lights, depth + 1);
    let refract_color = cast_ray(hit_point, refract_dir, spheres, lights, depth + 1);

    let mut diffuse_intensity = 0.0;
    let mut specular_intensity = 0.0;

    for light in lights {
        let light_dir = (light.position - hit_point).normalize();

        let shadow_orig = hit_point + n.mul_const((light_dir % n).signum() * SPHERES_SHD);
        let mut shadow_point = ZERO_VEC;
        let mut shadow_n = ZERO_VEC;

        let (intersects, ..) = spheres_intercects(shadow_orig, light_dir, &mut shadow_point, &mut shadow_n, spheres);

        if intersects && (shadow_point - shadow_orig).len() < (light_dir - hit_point).len() {
            continue;
        }

        diffuse_intensity += 0.0_f64.max(light_dir % n) * light.intensity;
        specular_intensity += (0.0_f64.max(reflect(light_dir, n) % dir))
            .powf(material.specular_exponent) * light.intensity;
    }

    material.diffuse_color
        .mul_const(diffuse_intensity)
        .mul_const(material.albedo.0)
        + ONE_VEC.mul_const(specular_intensity)
        .mul_const(material.albedo.1)
        + reflect_color.mul_const(material.albedo.2)
        + refract_color.mul_const(material.albedo.3)
}
}

pub fn render_spheres(
    framebuffer: &mut ImageMatrix,
    spheres: &Vec<Sphere>,
    lights: &Vec<Light>
) {
    let (w, h) = framebuffer.get_dimensions();
    let (wf, hf) = (w as f64, h as f64);
    let fov_2tan = (FOV / 2.0).tan();

    for j in 0..h {
        for i in 0..w {
            let x = (2.0 * (i as f64 + 0.5) / wf - 1.0) * fov_2tan * wf / hf;
            let y = -(2.0 * (j as f64 + 0.5) / hf - 1.0) * fov_2tan;

            let dir = Vec3::new(x, y, -1.0).normalize();

            framebuffer.put_pixel(i, j, render_impl::cast_ray(ZERO_VEC, dir, spheres, lights, 0))
        }
    }
}

