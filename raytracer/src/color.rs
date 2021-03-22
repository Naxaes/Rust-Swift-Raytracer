use crate::maths::Vec3;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    pub fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn add(&self, rhs: &Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
    pub fn add_with_alpha(&self, rhs: &Self) -> Self {
        Self::new_with_alpha(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
    pub fn mul_with_alpha(&self, rhs: &Self) -> Self {
        Self::new_with_alpha(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a * rhs.a)
    }
    pub fn lerp_with_alpha(&self, rhs: &Self, t: f32) -> Self {
        let l = 1.0-t;
        let r = t;

        Self::new_with_alpha(self.r * l, self.g * l, self.b * l, self.a * l)
            .mul_with_alpha(
                &Self::new_with_alpha(rhs.r * r, rhs.g * r, rhs.b * r, rhs.a * r)
            )
    }
}


impl From<Vec3> for Color {
    fn from(vec3: Vec3) -> Self {
        Self::new(vec3.x, vec3.y, vec3.z)
    }
}