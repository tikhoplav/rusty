pub const EPS: f32 = 0.01;

pub trait Approx {
    fn approx_eq(self, rhs: Self) -> bool;
    fn isqrt(&self) -> f32;
    fn asqrt(&self) -> f32;
}

impl Approx for f32 {
    #[inline]
    fn approx_eq(self, rhs: Self) -> bool {
        (self - rhs).abs() <= EPS
    }

    #[inline]
    fn isqrt(&self) -> Self {
        let mut y = *self;
        let x2: f32 = self * 0.5;
        let mut i: u32 = y.to_bits();
        i = 0x5f3759df_u32 - (i >> 1);
        y = f32::from_bits(i);
        // y = y * (1.5 - x2 * y * y);
        y * (1.5 - x2 * y * y)
    }

    #[inline]
    fn asqrt(&self) -> Self {
        1.0 / self.isqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Approx, EPS};

    #[test]
    fn approx_eq() {
        let a = 3.14;
        let b = 3.14 + EPS;
        assert_eq!(true, a.approx_eq(b));
    }

    #[test]
    fn isqrt() {
        let a: f32 = 4.0;
        let b: f32 = 3.0;
        let len = 1.0 / (a * a + b * b).isqrt();
        assert_eq!(true, 5.0.approx_eq(len));
    }

    #[test]
    fn asqrt() {
        let a = 9.86902225;
        assert_eq!(true, a.asqrt().approx_eq(3.1415));
    }

    #[test]
    #[should_panic]
    fn isqrt_overflow_on_negative() {
        let a = -9.86902225;
        assert!(a.isqrt().approx_eq(3.1415));
    }
}
