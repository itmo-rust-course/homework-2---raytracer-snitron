use image::{ImageBuffer, ImageResult, RgbImage};
use crate::render::constants::colour::BACKGROUND;
use crate::geometry::{sphere::Sphere, vec3::Vec3};

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

    pub fn put_pixel(&mut self, x: u32, y: u32, vec: &Vec3) {
        self.framebuffer.put_pixel(x, y, vec.into());
    }

    pub fn save_image(self, path: &str) -> ImageResult<()> {
        self.framebuffer.save(path)
    }
}

pub trait Renderable {
    fn render(&self, framebuffer: &mut ImageMatrix);
}

