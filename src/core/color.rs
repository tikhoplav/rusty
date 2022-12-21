use crate::core::Vec4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl PartialEq for Color {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2 && self.3 == rhs.3
    }
}

impl From<Vec4> for Color {
    #[inline]
    fn from(vec: Vec4) -> Self {
        Self(vec.0 as u8, vec.1 as u8, vec.2 as u8, vec.3 as u8)
    }
}

impl Into<Vec4> for Color {
    #[inline]
    fn into(self) -> Vec4 {
        Vec4(self.0 as f32, self.1 as f32, self.2 as f32, self.3 as f32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Color, Vec4};
    use std::fmt::{Debug, Formatter, Result};

    impl Debug for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Color({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }

    #[test]
    fn equal() {
        let a = Color(1, 0, 25, 255);
        let b = Color(1, 0, 25, 255);
        assert_eq!(a, b);
    }

    #[test]
    fn copy() {
        let a = Color(20, 20, 20, 255);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        let a = Color(20, 10, 0, 255);
        let b = Color(0, 20, 30, 0);
        let c = Color(20, 30, 30, 255);
        assert_eq!(
            c,
            Color::from(<Color as Into<Vec4>>::into(a) + <Color as Into<Vec4>>::into(b))
        );
    }

    #[test]
    fn sub() {
        let a = Color(20, 10, 0, 255);
        let b = Color(0, 10, 0, 0);
        let c = Color(20, 0, 0, 255);
        assert_eq!(
            c,
            Color::from(<Color as Into<Vec4>>::into(a) - <Color as Into<Vec4>>::into(b))
        );
    }

    #[test]
    fn mul() {
        let a = Color(20, 10, 0, 255);
        let b = 2.0;
        let c = Color(40, 20, 0, 255);
        assert_eq!(c, Color::from(<Color as Into<Vec4>>::into(a) * b));
    }

    #[test]
    fn div() {
        let a = Color(20, 10, 0, 255);
        let b = 2.0;
        let c = Color(10, 5, 0, 127);
        assert_eq!(c, Color::from(<Color as Into<Vec4>>::into(a) / b));
    }
}
