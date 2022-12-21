#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use rusty::core::{Mat4, Vec3, Vec4};
    use test::{black_box, Bencher};

    #[bench]
    fn equal(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        let y = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );

        b.iter(|| {
            for _ in 1..100 {
                black_box(x == y);
            }
        });
    }

    #[bench]
    fn native_eq(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        let y = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );

        b.iter(|| {
            for _ in 1..100 {
                black_box(
                    x.0 .0 == y.0 .0
                        && x.0 .1 == y.0 .1
                        && x.0 .2 == y.0 .2
                        && x.0 .3 == y.0 .3
                        && x.1 .0 == y.1 .0
                        && x.1 .1 == y.1 .1
                        && x.1 .2 == y.1 .2
                        && x.1 .3 == y.1 .3
                        && x.2 .0 == y.2 .0
                        && x.2 .1 == y.2 .1
                        && x.2 .2 == y.2 .2
                        && x.2 .3 == y.2 .3
                        && x.3 .0 == y.3 .0
                        && x.3 .1 == y.3 .1
                        && x.3 .2 == y.3 .2
                        && x.3 .3 == y.3 .3,
                );
            }
        });
    }

    #[bench]
    fn translation(b: &mut Bencher) {
        let x = Vec3(2.3, 3.6, 1.8);

        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::translation(x));
            }
        });
    }

    #[bench]
    fn look_at(b: &mut Bencher) {
        let x = Vec3(2.3, 3.6, 1.8);
        let y = Vec3(0.0, 0.0, 1.0);
        let z = Vec3(0.0, 0.0, 1.0);

        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::look_at(x, y, z));
            }
        });
    }

    #[bench]
    fn perspective(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::perspective(3.1415, 16.0 / 9.0, 0.01, 10.0));
            }
        });
    }

    #[bench]
    fn orthographic(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::orthographic(0.0, 1.0, 0.0, 1.0, 0.01, 10.0));
            }
        });
    }

    #[bench]
    fn frustum(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::frustum(0.0, 1.0, 0.0, 1.0, 0.01, 10.0));
            }
        });
    }

    #[bench]
    fn x_rotation(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::x_rotation(1.04719758));
            }
        });
    }

    #[bench]
    fn y_rotation(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::y_rotation(1.04719758));
            }
        });
    }

    #[bench]
    fn translate(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let y = Vec3(2.3, 3.6, 1.8);

        b.iter(|| {
            for _ in 1..100 {
                black_box(x.translate(y));
            }
        });
    }

    #[bench]
    fn non_optimized_translate(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let y = Vec3(2.3, 3.6, 1.8);

        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::translation(y).dot(x));
            }
        });
    }

    #[bench]
    fn x_rotate(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );

        b.iter(|| {
            for _ in 1..100 {
                black_box(x.x_rotate(1.04719758));
            }
        });
    }

    #[bench]
    fn non_optimized_x_rotate(b: &mut Bencher) {
        let x = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );

        b.iter(|| {
            for _ in 1..100 {
                black_box(Mat4::x_rotation(1.04719758).dot(x));
            }
        });
    }
}
