mod render;
mod geometry;

use geometry::vec3::Vec3;
use crate::geometry::Material;
use crate::geometry::sphere::{Sphere, RenderUtils};
use crate::render::{ImageMatrix};

fn main() {
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3));
    let red_rubber = Material::new(Vec3::new(0.3, 0.1, 0.1));

    let spheres = vec![
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 2.0, ivory)
    ];

    let mut image_matrix = ImageMatrix::new(1024, 768);
    RenderUtils::render_spheres(&mut image_matrix, &spheres);

    image_matrix.save_image("test3.png");
}
