use std::num::{Wrapping, NonZeroU32};

use crate::maths::{Vector, Vec3, Point, NVec3};


// ----------------- RAY ----------------------
#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: NVec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}



// ----------------- OTHER ----------------------
pub struct Random {
    state: Wrapping<u32>,
}

impl Random {
    pub fn new(seed: NonZeroU32) -> Random {
        Self { state: Wrapping(seed.get()) }
    }
    /// Random number between [0, 1].
    pub fn random_f32(&mut self) -> f32 {
        self.xor_shift_32() as f32 / u32::MAX as f32
    }
    /// Random number between [-1, 1].
    pub fn random_bilateral_f32(&mut self) -> f32 {
        self.random_f32() * 2.0 - 1.0
    }
    fn xor_shift_32(&mut self) -> u32
    {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x.0
    }
}


pub fn lerp<T>(a: T, b: T, t: f32) -> T
    where T: std::ops::Mul<f32, Output=T> + std::ops::Add<T, Output=T> {
    a*(1.0-t) + b*t
}


pub fn random_unit_sphere(random: &mut Random) -> NVec3 {
    NVec3::new(
        random.random_bilateral_f32(),
        random.random_bilateral_f32(),
        random.random_bilateral_f32(),
    )
}


// ----------------- FRAMEBUFFER ----------------------
#[derive(Debug, Clone)]
pub struct Framebuffer {
    max_color_value: usize,
    width:  usize,
    height: usize,
    pixels: Vec<Vec3>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let max_color_value = 255;

        let mut pixels : Vec<Vec3> = Vec::with_capacity(width * height);
        pixels.resize(width * height, Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        Self { max_color_value, width, height, pixels }
    }
}

impl std::ops::Index<[usize; 2]> for Framebuffer {
    type Output = Vec3;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, column] = index;
        &self.pixels[row * self.width + column]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Framebuffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, column] = index;
        &mut self.pixels[row * self.width + column]
    }
}


// ----------------- CAMERA ----------------------
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical:   Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        // Camera
        let viewport_height = 2.0;
        let viewport_width  = aspect_ratio * viewport_height;
        let focal_length    = 1.0;

        // Viewport
        let origin     = Point{ x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0 };
        let vertical   = Vec3{ x: 0.0, y: viewport_height, z: 0.0 };
        let lower_left_corner = origin - Vec3{ x: viewport_width / 2.0, y: viewport_height / 2.0, z: focal_length };

        Camera { origin, lower_left_corner, horizontal, vertical }
    }
    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin).normalize()
        }
    }
}


