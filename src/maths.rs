use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use std::convert::Into;

// Floating point hacks
// https://www.youtube.com/watch?v=ReTetN51r7A


/// A • B = |A| * |B| * cos x
pub fn dot() {

}

/// (X - Proj_L(X)) • V = 0     <=>
/// (X - c * V) • V = 0         <=>
/// X • V - c * V • V = 0       <=>
/// X • V = c * V • V           <=>
/// (X • V) / (V • V) = c
///
/// Proj_L(X) = c * V = ((X • V) / (V • V)) * V
///
pub fn project(v: Vec3, onto: Vec3) -> Vec3 {
    (onto.dot(&v) / onto.length_squared()) * onto
}


pub fn reflect(v: Vec3, n: NVec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}


pub fn refract(uv: NVec3, n: NVec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta  = NVec3::dot(&-uv, &n);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}


pub trait IVector : Sized + Copy {
    fn x(&self) -> f32;   fn r(&self) -> f32 { self.x() }
    fn y(&self) -> f32;   fn g(&self) -> f32 { self.y() }
    fn z(&self) -> f32;   fn b(&self) -> f32 { self.z() }
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn new_unchecked(x: f32, y: f32, z: f32) -> Self;

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }
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
use std::arch::x86_64::_rdrand16_step;

impl IVector for Vec3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    fn new_unchecked(x: f32, y: f32, z: f32) -> Self { Self::new(x, y, z) }
}
impl Vec3 {
    pub fn new_zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }

    pub fn normalize(&self) -> NVec3 { NVec3::new(self.x(), self.y(), self.z()) }

    pub fn dot(&self, rhs: &impl IVector) -> f32 { self.x()*rhs.x() + self.y()*rhs.y() + self.z()*rhs.z() }

    pub fn length_squared(&self) -> f32 { self.dot(self) }
    pub fn length(&self)         -> f32 { f32::sqrt(self.length_squared()) }

    /// A x B = |A| * |B| * sin x * n̂
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
              self.y*rhs.z - self.z*rhs.y,
            -(self.x*rhs.z - self.z*rhs.x),
              self.x*rhs.y - self.y*rhs.x
        )
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

impl IVector for NVec3 {
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
    fn new_unchecked(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
}

impl NVec3 {
    pub fn normalize(&self) -> NVec3 { NVec3::new(self.x(), self.y(), self.z()) }

    pub fn dot(&self, rhs: &impl IVector) -> f32 { self.x()*rhs.x() + self.y()*rhs.y() + self.z()*rhs.z() }

    pub fn length_squared(&self) -> f32 { 1.0 }
    pub fn length(&self)         -> f32 { 1.0 }

    /// A x B = |A| * |B| * sin x * n̂
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new_unchecked(
              self.y*rhs.z - self.z*rhs.y,
            -(self.x*rhs.z - self.z*rhs.x),
              self.x*rhs.y - self.y*rhs.x
        )
    }
}




mod vector {
    use super::*;
    
    fn add<T: IVector, U: IVector, V: IVector>(lhs: &T, rhs: &U) -> V { V::new_unchecked(lhs.x() + rhs.x(), lhs.y() + rhs.y(), lhs.z() + rhs.z()) }
    impl Add<Vec3>  for Vec3  { type Output = Vec3; fn add(self, rhs: Vec3)  -> Self::Output { add(&self, &rhs) } }
    impl Add<Vec3>  for NVec3 { type Output = Vec3; fn add(self, rhs: Vec3)  -> Self::Output { add(&self, &rhs) } }
    impl Add<NVec3> for Vec3  { type Output = Vec3; fn add(self, rhs: NVec3) -> Self::Output { add(&self, &rhs) } }
    impl Add<NVec3> for NVec3 { type Output = Vec3; fn add(self, rhs: NVec3) -> Self::Output { add(&self, &rhs) } }

    fn sub<T: IVector, U: IVector, V: IVector>(lhs: &T, rhs: &U) -> V { V::new_unchecked(lhs.x() - rhs.x(), lhs.y() - rhs.y(), lhs.z() - rhs.z()) }
    impl Sub<Vec3>  for Vec3  { type Output = Vec3; fn sub(self, rhs: Vec3)  -> Self::Output { sub(&self, &rhs) } }
    impl Sub<Vec3>  for NVec3 { type Output = Vec3; fn sub(self, rhs: Vec3)  -> Self::Output { sub(&self, &rhs) } }
    impl Sub<NVec3> for Vec3  { type Output = Vec3; fn sub(self, rhs: NVec3) -> Self::Output { sub(&self, &rhs) } }
    impl Sub<NVec3> for NVec3 { type Output = Vec3; fn sub(self, rhs: NVec3) -> Self::Output { sub(&self, &rhs) } }

    fn mul<T: IVector, U: IVector, V: IVector>(lhs: &T, rhs: &U) -> V { V::new_unchecked(lhs.x() * rhs.x(), lhs.y() * rhs.y(), lhs.z() * rhs.z()) }
    impl Mul<Vec3>  for Vec3  { type Output = Vec3; fn mul(self, rhs: Vec3)  -> Self::Output { mul(&self, &rhs) } }
    impl Mul<Vec3>  for NVec3 { type Output = Vec3; fn mul(self, rhs: Vec3)  -> Self::Output { mul(&self, &rhs) } }
    impl Mul<NVec3> for Vec3  { type Output = Vec3; fn mul(self, rhs: NVec3) -> Self::Output { mul(&self, &rhs) } }
    impl Mul<NVec3> for NVec3 { type Output = Vec3; fn mul(self, rhs: NVec3) -> Self::Output { mul(&self, &rhs) } }

    fn div<T: IVector, U: IVector, V: IVector>(lhs: &T, rhs: &U) -> V { V::new_unchecked(lhs.x() / rhs.x(), lhs.y() / rhs.y(), lhs.z() / rhs.z()) }
    impl Div<Vec3>  for Vec3  { type Output = Vec3; fn div(self, rhs: Vec3)  -> Self::Output { div(&self, &rhs) } }
    impl Div<Vec3>  for NVec3 { type Output = Vec3; fn div(self, rhs: Vec3)  -> Self::Output { div(&self, &rhs) } }
    impl Div<NVec3> for Vec3  { type Output = Vec3; fn div(self, rhs: NVec3) -> Self::Output { div(&self, &rhs) } }
    impl Div<NVec3> for NVec3 { type Output = Vec3; fn div(self, rhs: NVec3) -> Self::Output { div(&self, &rhs) } }


    impl AddAssign<Vec3>  for Vec3  { fn add_assign(&mut self, rhs: Vec3)  { *self = add(self, &rhs) } }
    impl AddAssign<Vec3>  for NVec3 { fn add_assign(&mut self, rhs: Vec3)  { *self = add(self, &rhs) } }
    impl AddAssign<NVec3> for Vec3  { fn add_assign(&mut self, rhs: NVec3) { *self = add(self, &rhs) } }
    impl AddAssign<NVec3> for NVec3 { fn add_assign(&mut self, rhs: NVec3) { *self = add(self, &rhs) } }

    impl SubAssign<Vec3>  for Vec3  { fn sub_assign(&mut self, rhs: Vec3)  { *self = sub(self, &rhs) } }
    impl SubAssign<Vec3>  for NVec3 { fn sub_assign(&mut self, rhs: Vec3)  { *self = sub(self, &rhs) } }
    impl SubAssign<NVec3> for Vec3  { fn sub_assign(&mut self, rhs: NVec3) { *self = sub(self, &rhs) } }
    impl SubAssign<NVec3> for NVec3 { fn sub_assign(&mut self, rhs: NVec3) { *self = sub(self, &rhs) } }

    impl MulAssign<Vec3>  for Vec3  { fn mul_assign(&mut self, rhs: Vec3)  { *self = mul(self, &rhs) } }
    impl MulAssign<Vec3>  for NVec3 { fn mul_assign(&mut self, rhs: Vec3)  { *self = mul(self, &rhs) } }
    impl MulAssign<NVec3> for Vec3  { fn mul_assign(&mut self, rhs: NVec3) { *self = mul(self, &rhs) } }
    impl MulAssign<NVec3> for NVec3 { fn mul_assign(&mut self, rhs: NVec3) { *self = mul(self, &rhs) } }

    impl DivAssign<Vec3>  for Vec3  { fn div_assign(&mut self, rhs: Vec3)  { *self = div(self, &rhs) } }
    impl DivAssign<Vec3>  for NVec3 { fn div_assign(&mut self, rhs: Vec3)  { *self = div(self, &rhs) } }
    impl DivAssign<NVec3> for Vec3  { fn div_assign(&mut self, rhs: NVec3) { *self = div(self, &rhs) } }
    impl DivAssign<NVec3> for NVec3 { fn div_assign(&mut self, rhs: NVec3) { *self = div(self, &rhs) } }


    fn add_scalar<T: IVector, U: IVector>(lhs: &T, rhs: f32) -> U { U::new_unchecked(lhs.x() + rhs, lhs.y() + rhs, lhs.z() + rhs) }
    impl Add<f32>   for Vec3  { type Output = Vec3; fn add(self, rhs: f32)   -> Self::Output { add_scalar(&self, rhs) } }
    impl Add<Vec3>  for f32   { type Output = Vec3; fn add(self, rhs: Vec3)  -> Self::Output { add_scalar(&rhs, self) } }
    impl Add<f32>   for NVec3 { type Output = Vec3; fn add(self, rhs: f32)   -> Self::Output { add_scalar(&self, rhs) } }
    impl Add<NVec3> for f32   { type Output = Vec3; fn add(self, rhs: NVec3) -> Self::Output { add_scalar(&rhs, self) } }

    fn sub_scalar<T: IVector, U: IVector>(lhs: &T, rhs: f32) -> U { U::new_unchecked(lhs.x() - rhs, lhs.y() - rhs, lhs.z() - rhs) }
    impl Sub<f32>   for Vec3  { type Output = Vec3; fn sub(self, rhs: f32)   -> Self::Output { sub_scalar(&self, rhs) } }
    impl Sub<Vec3>  for f32   { type Output = Vec3; fn sub(self, rhs: Vec3)  -> Self::Output { sub_scalar(&rhs, self) } }
    impl Sub<f32>   for NVec3 { type Output = Vec3; fn sub(self, rhs: f32)   -> Self::Output { sub_scalar(&self, rhs) } }
    impl Sub<NVec3> for f32   { type Output = Vec3; fn sub(self, rhs: NVec3) -> Self::Output { sub_scalar(&rhs, self) } }

    fn mul_scalar<T: IVector, U: IVector>(lhs: &T, rhs: f32) -> U { U::new_unchecked(lhs.x() * rhs, lhs.y() * rhs, lhs.z() * rhs) }
    impl Mul<f32>   for Vec3  { type Output = Vec3; fn mul(self, rhs: f32)   -> Self::Output { mul_scalar(&self, rhs) } }
    impl Mul<Vec3>  for f32   { type Output = Vec3; fn mul(self, rhs: Vec3)  -> Self::Output { mul_scalar(&rhs, self) } }
    impl Mul<f32>   for NVec3 { type Output = Vec3; fn mul(self, rhs: f32)   -> Self::Output { mul_scalar(&self, rhs) } }
    impl Mul<NVec3> for f32   { type Output = Vec3; fn mul(self, rhs: NVec3) -> Self::Output { mul_scalar(&rhs, self) } }

    fn div_scalar<T: IVector, U: IVector>(lhs: &T, rhs: f32) -> U { U::new_unchecked(lhs.x() / rhs, lhs.y() / rhs, lhs.z() / rhs) }
    impl Div<f32>   for Vec3  { type Output = Vec3; fn div(self, rhs: f32)   -> Self::Output { div_scalar(&self, rhs) } }
    impl Div<Vec3>  for f32   { type Output = Vec3; fn div(self, rhs: Vec3)  -> Self::Output { div_scalar(&rhs, self) } }
    impl Div<f32>   for NVec3 { type Output = Vec3; fn div(self, rhs: f32)   -> Self::Output { div_scalar(&self, rhs) } }
    impl Div<NVec3> for f32   { type Output = Vec3; fn div(self, rhs: NVec3) -> Self::Output { div_scalar(&rhs, self) } }


    fn negate<T: IVector>(lhs: &T) -> T { T::new_unchecked(-lhs.x(), -lhs.y(), -lhs.z()) }
    impl Neg for Vec3  { type Output = Self; fn neg(self) -> Self::Output { negate(&self) } }
    impl Neg for NVec3 { type Output = Self; fn neg(self) -> Self::Output { negate(&self) } }


    impl From<Vec3>  for NVec3 { fn from(other: Vec3)  -> Self { Self::new(other.x(), other.y(), other.z()) } }
    impl From<NVec3> for Vec3  { fn from(other: NVec3) -> Self { Self::new(other.x(), other.y(), other.z()) } }
}




#[cfg(test)]
mod tests {
    use super::*;

    fn vec3_equal(result: Vec3, expected: Vec3) {
        let diff = result - expected;
        assert!(diff.near_zero(),
            "\n\tGot      {:?}\
             \n\tExpected {:?}\
             \n\tDiff     {:?}",
            result, expected, diff
        );
    }

    #[test]
    fn test_negate() {
        vec3_equal(
            -Vec3::new(1.0,   2.0,  3.0),
             Vec3::new(-1.0, -2.0, -3.0),
        );
    }

    #[test]
    fn test_reflect() {
        vec3_equal(
            reflect(Vec3::new(1.0, 0.0, -1.0), NVec3::new(0.0, 0.0, 1.0)),
            Vec3::new(1.0, 0.0, 1.0)
        );
    }

    #[test]
    fn test_project() {
        vec3_equal(
            project(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            Vec3::new(1.0, 0.0, 0.0),
        );
        vec3_equal(
            project(Vec3::new(2.0, 3.0, 0.0), Vec3::new(2.0, 1.0, 0.0)),
            Vec3::new(2.8, 1.4, 0.0),
        );
    }

    #[test]
    fn test_cross() {
        vec3_equal(
            cross(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            Vec3::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn test_refract() {
        let a = NVec3::new(1.0, 0.0, -1.0);
        vec3_equal(
            refract(a, NVec3::new(0.0, 0.0, 1.0), 1.0),
            a.into()
        );
    }
}