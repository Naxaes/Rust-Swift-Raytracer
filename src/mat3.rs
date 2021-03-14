use std::fmt;

use crate::maths::{Vec3, IVector, X_AXIS, Y_AXIS, Z_AXIS};
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
    r1: Vec3,
    r2: Vec3,
    r3: Vec3,
}
impl Mat3 {
    pub fn new(r1: Vec3, r2: Vec3, r3: Vec3) -> Self {
        Self { r1, r2, r3 }
    }
    pub fn identity() -> Self {
        Self { r1: X_AXIS.into(), r2: Y_AXIS.into(), r3: Z_AXIS.into() }
    }
    pub fn equals(&self, rhs: &Self) -> bool {
        (self.r1 - rhs.r1).near_zero() &&
        (self.r2 - rhs.r2).near_zero() &&
        (self.r3 - rhs.r3).near_zero()
    }
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let r1 = self.r1 * rhs;
        let r2 = self.r2 * rhs;
        let r3 = self.r3 * rhs;

        Self { r1, r2, r3 }
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        let a = rhs.transpose();

        let r1x = self.r1.dot(&a.r1);
        let r1y = self.r1.dot(&a.r2);
        let r1z = self.r1.dot(&a.r3);

        let r2x = self.r2.dot(&a.r1);
        let r2y = self.r2.dot(&a.r2);
        let r2z = self.r2.dot(&a.r3);

        let r3x = self.r3.dot(&a.r1);
        let r3y = self.r3.dot(&a.r2);
        let r3z = self.r3.dot(&a.r3);

        let r1 = Vec3::new(r1x, r1y, r1z);
        let r2 = Vec3::new(r2x, r2y, r2z);
        let r3 = Vec3::new(r3x, r3y, r3z);

        Self { r1, r2, r3 }
    }
    pub fn mul_vec3(&self, rhs: &Vec3) -> Vec3 {
        rhs.clone()
    }
    #[allow(unused_parens)]
    pub fn cofactor(&self) -> Self {
        // Cramer's rule: https://en.wikipedia.org/wiki/Cramer%27s_rule
        let r1x =  (self.r2.y*self.r3.z - self.r3.y*self.r2.z);
        let r1y = -(self.r2.x*self.r3.z - self.r3.x*self.r2.z);
        let r1z =  (self.r2.x*self.r3.y - self.r3.x*self.r2.y);

        let r2x = -(self.r1.y*self.r3.z - self.r3.y*self.r1.z);
        let r2y =  (self.r1.x*self.r3.z - self.r3.x*self.r1.z);
        let r2z = -(self.r1.x*self.r3.y - self.r3.x*self.r1.y);

        let r3x =  (self.r1.y*self.r2.z - self.r2.y*self.r1.z);
        let r3y = -(self.r1.x*self.r2.z - self.r2.x*self.r1.z);
        let r3z =  (self.r1.x*self.r2.y - self.r2.x*self.r1.y);

        Self {
            r1: Vec3::new(r1x, r1y, r1z),
            r2: Vec3::new(r2x, r2y, r2z),
            r3: Vec3::new(r3x, r3y, r3z),
        }
    }
    pub fn adjugate(&self) -> Self {
        self.cofactor().transpose()
    }
    #[allow(unused_parens)]
    pub fn inverse(&self) -> Option<Self> {
        // Then the inverse of A is the transpose of the cofactor matrix (adjugate) times
        // the reciprocal of the determinant of A, i.e. A^(-1) = (1 / det(A)) * C^T.
        //     let determinant = self.det();
        //     if determinant == 0.0 {
        //         None
        //     } else {
        //         Some(self.adjugate().mul_scalar(1.0 / determinant))
        //     }

        // Cofactor (here `cf`).
        let cf_r1x =  (self.r2.y*self.r3.z - self.r3.y*self.r2.z);
        let cf_r1y = -(self.r2.x*self.r3.z - self.r3.x*self.r2.z);
        let cf_r1z =  (self.r2.x*self.r3.y - self.r3.x*self.r2.y);

        let cf_r2x = -(self.r1.y*self.r3.z - self.r3.y*self.r1.z);
        let cf_r2y =  (self.r1.x*self.r3.z - self.r3.x*self.r1.z);
        let cf_r2z = -(self.r1.x*self.r3.y - self.r3.x*self.r1.y);

        let cf_r3x =  (self.r1.y*self.r2.z - self.r2.y*self.r1.z);
        let cf_r3y = -(self.r1.x*self.r2.z - self.r2.x*self.r1.z);
        let cf_r3z =  (self.r1.x*self.r2.y - self.r2.x*self.r1.y);

        // Adjugate = Transposed cofactors.
        let adjugate = Mat3::new(
            Vec3::new(cf_r1x, cf_r2x, cf_r3x),
            Vec3::new(cf_r1y, cf_r2y, cf_r3y),
            Vec3::new(cf_r1z, cf_r2z, cf_r3z),
        );

        let determinant = self.r1.x*cf_r1x + self.r1.y*cf_r1y + self.r1.z*cf_r1z;
        if determinant == 0.0 {
            None
        } else {
            Some(adjugate.mul_scalar(1.0 / determinant))
        }
    }

    pub fn det(&self) -> f32 {
        self.r1.x * (self.r2.y*self.r3.z - self.r3.y*self.r2.z)
       -self.r1.y * (self.r2.x*self.r3.z - self.r3.x*self.r2.z)
       +self.r1.z * (self.r2.x*self.r3.y - self.r3.x*self.r2.y)
    }

    pub fn transpose(&self) -> Self {
        let r1 = Vec3::new(self.r1.x, self.r2.x, self.r3.x);
        let r2 = Vec3::new(self.r1.y, self.r2.y, self.r3.y);
        let r3 = Vec3::new(self.r1.z, self.r2.z, self.r3.z);

        Self { r1, r2, r3 }
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
           "[({:.2}, {:.2}, {:.2}), ({:.2}, {:.2}, {:.2}), ({:.2}, {:.2}, {:.2})]",
           self.r1.x, self.r1.y, self.r1.z,
           self.r2.x, self.r2.y, self.r2.z,
           self.r3.x, self.r3.y, self.r3.z
       )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_identity() {
        let identity = Mat3::identity();
        let a = Mat3::new(
            Vec3::new( 2.0,  1.5, -1.1),
            Vec3::new( 6.0, -3.5, -2.1),
            Vec3::new(-1.2,  3.2, -5.3),
        );
        assert!(identity.mul(&a).equals(&a));
        assert!(a.mul(&identity).equals(&a));
    }

    #[test]
    fn mul() {
        let a = Mat3::new(
            Vec3::new(-5.0,   2.0,  0.0),
            Vec3::new( 6.0,  -3.0,  3.0),
            Vec3::new( 34.0, -4.0, -2.0),
        );
        let b = Mat3::new(
            Vec3::new( 2.0,  2.5, -1.0),
            Vec3::new( 6.0, -4.0, -2.0),
            Vec3::new(-1.0,  3.0, -4.5),
        );
        let c = Mat3::new(
            Vec3::new( 2.0, -41.0/2.0, 1.0),
            Vec3::new(-9.0,  36.0,    -27.0/2.0),
            Vec3::new( 46.0, 95.0,    -17.0),

        );
        assert!(a.mul(&b).equals(&c));
        assert!(!b.mul(&a).equals(&a));
    }

    #[test]
    fn transpose_identity() {
        assert!(Mat3::identity().transpose().equals(&Mat3::identity()));
    }

    #[test]
    fn transpose() {
        let a = Mat3::new(
            Vec3::new(-5.0,   2.0,  0.0),
            Vec3::new( 6.0,  -3.0,  3.0),
            Vec3::new( 34.0, -4.0, -2.0),
        );
        let b = Mat3::new(
            Vec3::new(-5.0,  6.0, 34.0),
            Vec3::new( 2.0, -3.0, -4.0),
            Vec3::new( 0.0,  3.0, -2.0),
        );
        assert!(a.transpose().equals(&b));
    }

    #[test]
    fn determinant() {
        let a = Mat3::new(
            Vec3::new(-5.0,   2.0,  0.0),
            Vec3::new( 6.0,  -3.0,  3.0),
            Vec3::new( 34.0, -4.0, -2.0),
        );
        assert!((a.det() - 138.0) < 1e-8, "{} != {}", a.det(), 138);
    }

    #[test]
    fn cofactor() {
        let a = Mat3::new(
            Vec3::new(3.0, 0.0,  2.0),
            Vec3::new(2.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0,  1.0),
        );
        let b = Mat3::new(
            Vec3::new(2.0, -2.0,  2.0),
            Vec3::new(2.0,  3.0, -3.0),
            Vec3::new(0.0, 10.0,  0.0),
        );

        assert!(a.cofactor().equals(&b), "\n{}\n{}\n", a.cofactor(), b);
    }

    #[test]
    fn inverse() {
        let a = Mat3::new(
            Vec3::new(3.0, 0.0,  2.0),
            Vec3::new(2.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0,  1.0),
        );
        let b = Mat3::new(
            Vec3::new( 0.2,  0.2, 0.0),
            Vec3::new(-0.2,  0.3, 1.0),
            Vec3::new( 0.2, -0.3, 0.0),
        );

        assert!(a.inverse().unwrap().equals(&b), "\n{}\n{}\n", a.inverse().unwrap(), b);
    }
}