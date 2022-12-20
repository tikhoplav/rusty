#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use rusty::approx::{Approx, EPS};
    use test::{black_box, Bencher};

    #[bench]
    fn approx_eq(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        let y: f32 = 3.1416;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x.approx_eq(y));
            }
        });
    }

    #[bench]
    fn no_abs_approx_eq(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        let y: f32 = 3.1416;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x - y <= EPS && x - y >= EPS);
            }
        });
    }

    #[bench]
    fn native_eq(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        let y: f32 = 3.1416;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x.eq(&y));
            }
        });
    }

    #[bench]
    fn approx_isqrt(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x.isqrt());
            }
        });
    }

    #[bench]
    fn native_isqrt(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        b.iter(|| {
            for _ in 1..100 {
                black_box(1.0 / x.sqrt());
            }
        });
    }

    #[bench]
    fn asqrt(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x.asqrt());
            }
        });
    }

    #[bench]
    fn native_sqrt(b: &mut Bencher) {
        let x: f32 = 3.14159265359;
        b.iter(|| {
            for _ in 1..100 {
                black_box(x.sqrt());
            }
        });
    }
}
