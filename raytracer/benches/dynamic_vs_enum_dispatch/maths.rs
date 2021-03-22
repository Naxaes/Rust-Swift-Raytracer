use std::ops::{Add, Sub, Mul, Div};
use std::convert::Into;


pub fn reflect(v: NVec3, n: NVec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}


pub trait Vector : Div<f32, Output=Vec3> + Sized + Copy {
    fn x(&self) -> f32;   fn r(&self) -> f32 { self.x() }
    fn y(&self) -> f32;   fn g(&self) -> f32 { self.y() }
    fn z(&self) -> f32;   fn b(&self) -> f32 { self.z() }
    fn new(x: f32, y: f32, z: f32) -> Self;

    fn normalize(&self) -> NVec3 { NVec3::new(self.x(), self.y(), self.z()) }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    fn dot(&self, rhs: &impl Vector) -> f32 { self.x()*rhs.x() + self.y()*rhs.y() + self.z()*rhs.z() }

    fn length_squared(&self) -> f32 { self.dot(self) }
    fn length(&self)         -> f32 { f32::sqrt(self.length_squared()) }
}


pub const X_AXIS: NVec3 = NVec3 {x: 1.0, y: 0.0, z: 0.0};
pub const Y_AXIS: NVec3 = NVec3 {x: 0.0, y: 1.0, z: 0.0};
pub const Z_AXIS: NVec3 = NVec3 {x: 0.0, y: 0.0, z: 1.0};


// ---- VECTOR ----

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub use Vec3 as Point;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}


impl Vector for Vec3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}


// ---- NORMALIZED VECTOR ----
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NVec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector for NVec3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
    fn new(x: f32, y: f32, z: f32) -> Self {
        let length = f32::sqrt(x*x + y*y + z*z);
        Self {
            x: x/length,
            y: y/length,
            z: z/length,
        }
    }

    fn normalize(&self)      -> NVec3 { NVec3::new(self.x(), self.y(), self.z()) }
    fn length_squared(&self) -> f32 { 1.0 }
    fn length(&self)         -> f32 { 1.0 }
}

impl Into<Vec3> for NVec3 {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Add for NVec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<Vec3> for NVec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for NVec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<Vec3> for NVec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div<f32> for NVec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<NVec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: NVec3) -> Self::Output {
        Self::Output::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl Mul<f32> for NVec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}