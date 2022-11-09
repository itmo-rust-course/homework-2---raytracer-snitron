use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Div, Mul, Sub, Rem};
use image::Rgb;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    y: f64,
    z: f64
}

pub const ZERO_VEC: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const ONE_VEC: Vec3 = Vec3::new(1.0, 1.0, 1.0);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();

        Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
    }

    pub fn mul_const(&self, c: f64) -> Vec3 {
        Vec3 { x: self.x * c, y: self.y * c, z: self.z * c }
    }

    fn max_component(&self) -> f64 {
        self.x.max(self.y.max(self.z))
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

// Means scalar product
impl Rem for Vec3 {
    type Output = f64;

    fn rem(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Into<Rgb<u8>> for Vec3 {
    fn into(self) -> Rgb<u8> {
        let max = self.max_component();
        let corrector = 1.0 ;

        Rgb([(self.x * corrector * 255.0) as u8, (self.y * corrector * 255.0) as u8, (self.z * corrector* 255.0) as u8])
    }
}

impl<'a> Into<Rgb<u8>> for &'a Vec3 {
    fn into(self) -> Rgb<u8> {
        let max = self.max_component();
        let corrector = 1.0;

        Rgb([(self.x * corrector * 255.0) as u8, (self.y * corrector * 255.0) as u8, (self.z * corrector* 255.0) as u8])
    }
}