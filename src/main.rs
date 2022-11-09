mod render;
mod geometry;

use geometry::vec3::Vec3;
use crate::geometry::light::Light;
use crate::geometry::Material;
use crate::geometry::sphere::{Sphere, RenderUtils};
use crate::render::{ImageMatrix};

fn main() {
    let ivory = Material::new((0.6, 0.3, 0.1), Vec3::new(0.4, 0.4, 0.3), 50.0);
    let red_rubber = Material::new((0.9, 0.1, 0.0),Vec3::new(0.3, 0.1, 0.1), 10.0);
    let mirror = Material::new((0.0, 10.0, 0.8),Vec3::new(1.0, 1.0, 1.0), 1425.0);


    let spheres = vec![
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, mirror),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, mirror)
    ];

    let lights = vec![
        Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(Vec3::new(30.0, 50.0, -25.0), 1.8),
        Light::new(Vec3::new(30.0, 20.0, 30.0), 1.7),
        //Light::new(Vec3::new(10.0, 15.0, 30.0), 2.7)

    ];

    let mut image_matrix = ImageMatrix::new(1024, 768);
    RenderUtils::render_spheres(&mut image_matrix, &spheres, &lights);

    image_matrix.save_image("test7.png");
}
