mod render;
mod geometry;

use geometry::vec3::Vec3;
use crate::geometry::sphere::Sphere;
use crate::render::{ImageMatrix, Renderable};

fn generate_bullshit(w: u32, h: u32) -> ImageMatrix {
    let mut image_matrix = ImageMatrix::new(w, h);

    for i in 0..w {
        for j in 0..h {
            image_matrix.put_pixel(i, j, &Vec3::new((i as f64) / w as f64,  (j as f64) / h as f64, 0.0));
        }
    }

    image_matrix
}

fn main() {
    let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0);
    let mut image_matrix = ImageMatrix::new(1024, 768);
    sphere.render(&mut image_matrix);
    image_matrix.save_image("test2.png");
}
