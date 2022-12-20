use crate::approx::Approx;
use crate::vec4::Vec4;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Vec3 is a data structure that represent 3-component vector. It can be used
/// both for storing state data in memory as well as to perform calculations.
/// For performance sake the overflow checks are omitted for math operations as
/// it is user code responsibility to pick a suitable data type for components
/// for a specific purpose.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0.approx_eq(rhs.0) && self.1.approx_eq(rhs.1) && self.2.approx_eq(rhs.2)
    }
}

impl From<Vec4> for Vec3 {
    #[inline]
    fn from(vec: Vec4) -> Self {
        Self(vec.0, vec.1, vec.2)
    }
}

impl Into<Vec4> for Vec3 {
    #[inline]
    fn into(self) -> Vec4 {
        Vec4(self.0, self.1, self.2, 1.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Scalar multiplication
impl<T: Into<f32> + Copy> Mul<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self(
            self.0 * rhs.into(),
            self.1 * rhs.into(),
            self.2 * rhs.into(),
        )
    }
}

/// Scalar multiplication
impl<T: Into<f32> + Copy> MulAssign<T> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

/// Scalar division
impl<T: Into<f32> + Copy> Div<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self(
            self.0 / rhs.into(),
            self.1 / rhs.into(),
            self.2 / rhs.into(),
        )
    }
}

/// Scalar division
impl<T: Into<f32> + Copy> DivAssign<T> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Vec3 {
    /// Squared length of the vector
    #[inline]
    pub fn len2(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Length of the vector
    #[inline]
    pub fn len(self) -> f32 {
        self.len2().sqrt()
    }

    /// Normalize vector turing it's length to 1
    #[inline]
    pub fn normalize(self) -> Self {
        let isqrt = self.len2().isqrt();
        Self(self.0 * isqrt, self.1 * isqrt, self.2 * isqrt)
    }

    /// Cross product of two 3D vectors
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    /// Dot product of two 3D vectors
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    use std::fmt::{Debug, Formatter, Result};

    impl Debug for Vec3 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
        }
    }

    #[test]
    fn equal() {
        let a = Vec3(0.32, 0.64, 0.0);
        let b = Vec3(0.32, 0.64, 0.0);
        assert_eq!(a, b);
    }

    #[test]
    fn copy() {
        let a = Vec3(0.32, 0.64, 0.0);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        let a = Vec3(0.1, 255.0, 0.2);
        let b = Vec3(255.0, 0.1, 0.2);
        let c = Vec3(255.1, 255.1, 0.4);
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Vec3(8.0, 16.0, 32.0);
        let b = Vec3(8.0, 8.0, 8.0);
        let c = Vec3(0.0, 8.0, 24.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = Vec3(8.0, 16.0, 32.0);
        let b = 2.0;
        let c = Vec3(16.0, 32.0, 64.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn div() {
        let a = Vec3(8.0, 16.0, 32.0);
        let b = 2.0;
        let c = Vec3(4.0, 8.0, 16.0);
        assert_eq!(a / b, c);
    }

    #[test]
    fn len2() {
        let a = Vec3(3.0, 4.0, 0.0);
        assert_eq!(25.0, a.len2());
    }

    #[test]
    fn len() {
        let a = Vec3(3.0, 4.0, 0.0);
        assert_eq!(5.0, a.len());
    }

    #[test]
    fn normalize() {
        let a = Vec3(3.0, 4.0, 0.0);
        let i = Vec3(0.6, 0.8, 0.0);
        assert_eq!(i, a.normalize());
    }

    #[test]
    fn cross() {
        let a = Vec3(1.3, 2.1, 3.4);
        let b = Vec3(2.2, 1.8, 3.6);
        let c = Vec3(1.44, 2.8, -2.28);
        assert_eq!(c, a.cross(b));
    }

    #[test]
    fn dot() {
        let a = Vec3(1.0, 3.0, -5.0);
        let b = Vec3(4.0, -2.0, -1.0);
        assert_eq!(3.0, a.dot(b));
    }
}
