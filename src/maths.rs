#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub use Vec3 as Point;

pub trait Vector : std::ops::Div<f32, Output=Self> + Sized + Copy {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn new(x: f32, y: f32, z: f32) -> Self;

    fn x_axis() -> NVec3 { NVec3{ x: 1.0, y: 0.0, z: 0.0 } }
    fn y_axis() -> NVec3 { NVec3{ x: 0.0, y: 1.0, z: 0.0 } }
    fn z_axis() -> NVec3 { NVec3{ x: 0.0, y: 0.0, z: 1.0 } }

    fn normalize(&self) -> NVec3 { NVec3::new(self.x(), self.y(), self.z()) }

    fn dot(&self, rhs: &impl Vector) -> f32 {
        self.x()*rhs.x() + self.y()*rhs.y() + self.z()*rhs.z()
    }

    fn length_squared(&self) -> f32 { self.dot(self) }
    fn length(&self)         -> f32 { f32::sqrt(self.length_squared()) }
}


impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output { x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NVec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl NVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let length = f32::sqrt(x*x + y*y + z*z);
        Self {
            x: x/length,
            y: y/length,
            z: z/length,
        }
    }
}


impl Vector for NVec3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn length_squared(&self) -> f32 { 1.0 }
    fn length(&self)         -> f32 { 1.0 }
    fn normalize(&self)      -> NVec3 { self.clone() }
}

impl std::ops::Div<f32> for NVec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl std::ops::Mul<NVec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: NVec3) -> Self::Output {
        Self::Output { x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}
impl std::ops::Mul<f32> for NVec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}