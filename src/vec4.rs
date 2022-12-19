use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Vec4 is a data structure that represent 4-component vector. It can be used
/// both for storing state data in memory as well as to perform calculations.
/// For performance sake the overflow checks are omitted for math operations as
/// it is user code responsibility to pick a suitable data type for components
/// for a specific purpose.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2 && self.3 == rhs.3
    }
}

impl Add for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Scalar multiplication
impl<T: Into<f32> + Copy> Mul<T> for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self(
            self.0 * rhs.into(),
            self.1 * rhs.into(),
            self.2 * rhs.into(),
            self.3 * rhs.into(),
        )
    }
}

/// Scalar multiplication
impl<T: Into<f32> + Copy> MulAssign<T> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

/// Scalar division
impl<T: Into<f32> + Copy> Div<T> for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self(
            self.0 / rhs.into(),
            self.1 / rhs.into(),
            self.2 / rhs.into(),
            self.3 / rhs.into(),
        )
    }
}

/// Scalar division
impl<T: Into<f32> + Copy> DivAssign<T> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::vec4::Vec4;
    use std::fmt::{Debug, Formatter, Result};

    impl Debug for Vec4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Vec4({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }

    #[test]
    fn equal() {
        let a = Vec4(0.32, 0.64, 0.0, 0.1);
        let b = Vec4(0.32, 0.64, 0.0, 0.1);
        assert_eq!(a, b);
    }

    #[test]
    fn copy() {
        let a = Vec4(0.32, 0.64, 0.0, 0.1);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        let a = Vec4(0.1, 255.0, 0.2, 0.0);
        let b = Vec4(255.0, 0.1, 0.2, 0.0);
        let c = Vec4(255.1, 255.1, 0.4, 0.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Vec4(8.0, 16.0, 32.0, 0.0);
        let b = Vec4(8.0, 8.0, 8.0, 8.0);
        let c = Vec4(0.0, 8.0, 24.0, -8.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = Vec4(8.0, 16.0, 32.0, 0.0);
        let b = 2.0;
        let c = Vec4(16.0, 32.0, 64.0, 0.0);
        assert_eq!(a * b, c);
    }

    #[test]
    fn div() {
        let a = Vec4(8.0, 16.0, 32.0, 0.0);
        let b = 2.0;
        let c = Vec4(4.0, 8.0, 16.0, 0.0);
        assert_eq!(a / b, c);
    }
}
