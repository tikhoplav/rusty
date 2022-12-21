use crate::core::{Vec3, Vec4};
use std::f32::consts::FRAC_PI_2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

impl PartialEq for Mat4 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2 && self.3 == rhs.3
    }
}

impl Mat4 {
    /// Return a 4x4 matrix with diagonal elements set to 1 and the res  
    // set to 0, which correspond with a math Identity matrix definition.
    #[inline]
    pub fn identity() -> Self {
        Self(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Return a translation matrix.
    #[inline]
    pub fn translation(t: Vec3) -> Self {
        Self(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(t.0, t.1, t.2, 1.0),
        )
    }

    /// Return a matrix that represent transformation from the origin to a  
    /// place and orientation in the World.
    #[inline]
    pub fn look_at(origin: Vec3, target: Vec3, up: Vec3) -> Self {
        let z = (origin - target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x).normalize();
        Self(
            Vec4(x.0, x.1, x.2, 0.0),
            Vec4(y.0, y.1, y.2, 0.0),
            Vec4(z.0, z.1, z.2, 0.0),
            Vec4(origin.0, origin.1, origin.2, 1.0),
        )
    }

    /// Computes a 4-by-4 perspective transformation matrix given the  
    /// angular height of the frustum, the aspect ratio, and the near and  
    /// far clipping planes. The arguments define a frustum extending in the  
    /// negative z direction. The given angle is the vertical angle of the  
    /// frustum, and the horizontal angle is determined to produce the given  
    /// aspect ratio. The arguments near and far are the distances to the  
    /// near and far clipping planes. Note that near and far are not z  
    /// coordinates, but rather distances along the negative z-axis. The  
    /// matrix generated sends the viewing frustum to the unit box. Assume  
    /// a unit box extending from -1 to 1 in the x and y dimensions and from  
    /// -1 to 1 in the z dimension.
    #[inline]
    pub fn perspective(fow: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = (FRAC_PI_2 - 0.5 * fow).tan();
        let r = 1.0 / (near - far);
        Self(
            Vec4(f / aspect, 0.0, 0.0, 0.0),
            Vec4(0.0, f, 0.0, 0.0),
            Vec4(0.0, 0.0, (near + far) * r, -1.0),
            Vec4(0.0, 0.0, 2.0 * near * far * r, 0.0),
        )
    }

    /// Computes a 4-by-4 orthographic projection matrix given the  
    /// coordinates of the planes defining the axis-aligned, box-shaped  
    /// viewing volume.
    /// The matrix generated sends that box to the unit box. Note that  
    /// although left and right are x coordinates and bottom and top are y  
    /// coordinates, near and far are not z coordinates, but rather they are  
    /// distances along the negative z-axis. Assume a unit box extending  
    /// from -1 to 1 in the x and y dimensions and from -1 to 1 in the z  
    /// dimension.
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self(
            Vec4(2.0 / (right - left), 0.0, 0.0, 0.0),
            Vec4(0.0, 2.0 / (top - bottom), 0.0, 0.0),
            Vec4(0.0, 0.0, 2.0 / (near - far), 0.0),
            Vec4(
                (left + right) / (left - right),
                (bottom + top) / (bottom - top),
                (near + far) / (near - far),
                1.0,
            ),
        )
    }

    /// Computes a 4-by-4 perspective transformation matrix given the left,  
    /// right, top, bottom, near and far clipping planes. The arguments  
    /// define a frustum extending in the negative z direction. The  
    /// arguments near and far are the distances to the near and far  
    /// clipping planes. Note that near and far are not z coordinates, but  
    /// rather distances along the negative z-axis.
    /// The matrix generated sends the viewing frustum to the unit box.  
    /// Assume a unit box extending from -1 to 1 in the x and y dimensions  
    /// and from -1 to 1 in the z dimension.
    #[inline]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let dx = right - left;
        let dy = top - bottom;
        let dz = far - near;
        Self(
            Vec4(2.0 * near / dx, 0.0, 0.0, 0.0),
            Vec4(0.0, 2.0 * near / dy, 0.0, 0.0),
            Vec4(
                (left + right) / dx,
                (top + bottom) / dy,
                -(far + near) / dz,
                -1.0,
            ),
            Vec4(0.0, 0.0, -2.0 * near * far / dz, 0.0),
        )
    }

    /// Return an x-rotation matrix.
    #[inline]
    pub fn x_rotation(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, cos, sin, 0.0),
            Vec4(0.0, -sin, cos, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Return an y-rotation matrix.
    #[inline]
    pub fn y_rotation(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            Vec4(cos, 0.0, -sin, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(sin, 0.0, cos, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Return an z-rotation matrix.
    #[inline]
    pub fn z_rotation(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            Vec4(cos, sin, 0.0, 0.0),
            Vec4(-sin, cos, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Return a rotation matrix around an arbitrary axis.  
    /// Assume that provided axis is already normalized.
    #[inline]
    pub fn axis_rotation(axis: Vec3, angle: f32) -> Self {
        let Vec3 (x, y, z) = axis;
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let sin = angle.sin();
        let cos = angle.cos();
        let p = 1.0 - cos;
        Self(
            Vec4(
                xx + (1.0 - xx) * cos,
                x * y * p + z * sin,
                x * z * p - y * sin,
                0.0,
            ),
            Vec4(
                x * y * p - z * sin,
                yy + (1.0 - yy) * cos,
                y * z * p + x * sin,
                0.0,
            ),
            Vec4(
                x * z * p + y * sin,
                y * z * p - x * sin,
                zz + (1.0 - zz) * cos,
                0.0,
            ),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Calculate a dot product of two matrices.
    #[inline]
    pub fn dot(self, rhs: Self) -> Self {
        Self(
            Vec4(
                self.0 .0 * rhs.0 .0
                    + self.0 .1 * rhs.1 .0
                    + self.0 .2 * rhs.2 .0
                    + self.0 .3 * rhs.3 .0,
                self.0 .0 * rhs.0 .1
                    + self.0 .1 * rhs.1 .1
                    + self.0 .2 * rhs.2 .1
                    + self.0 .3 * rhs.3 .1,
                self.0 .0 * rhs.0 .2
                    + self.0 .1 * rhs.1 .2
                    + self.0 .2 * rhs.2 .2
                    + self.0 .3 * rhs.3 .2,
                self.0 .0 * rhs.0 .3
                    + self.0 .1 * rhs.1 .3
                    + self.0 .2 * rhs.2 .3
                    + self.0 .3 * rhs.3 .3,
            ),
            Vec4(
                self.1 .0 * rhs.0 .0
                    + self.1 .1 * rhs.1 .0
                    + self.1 .2 * rhs.2 .0
                    + self.1 .3 * rhs.3 .0,
                self.1 .0 * rhs.0 .1
                    + self.1 .1 * rhs.1 .1
                    + self.1 .2 * rhs.2 .1
                    + self.1 .3 * rhs.3 .1,
                self.1 .0 * rhs.0 .2
                    + self.1 .1 * rhs.1 .2
                    + self.1 .2 * rhs.2 .2
                    + self.1 .3 * rhs.3 .2,
                self.1 .0 * rhs.0 .3
                    + self.1 .1 * rhs.1 .3
                    + self.1 .2 * rhs.2 .3
                    + self.1 .3 * rhs.3 .3,
            ),
            Vec4(
                self.2 .0 * rhs.0 .0
                    + self.2 .1 * rhs.1 .0
                    + self.2 .2 * rhs.2 .0
                    + self.2 .3 * rhs.3 .0,
                self.2 .0 * rhs.0 .1
                    + self.2 .1 * rhs.1 .1
                    + self.2 .2 * rhs.2 .1
                    + self.2 .3 * rhs.3 .1,
                self.2 .0 * rhs.0 .2
                    + self.2 .1 * rhs.1 .2
                    + self.2 .2 * rhs.2 .2
                    + self.2 .3 * rhs.3 .2,
                self.2 .0 * rhs.0 .3
                    + self.2 .1 * rhs.1 .3
                    + self.2 .2 * rhs.2 .3
                    + self.2 .3 * rhs.3 .3,
            ),
            Vec4(
                self.3 .0 * rhs.0 .0
                    + self.3 .1 * rhs.1 .0
                    + self.3 .2 * rhs.2 .0
                    + self.3 .3 * rhs.3 .0,
                self.3 .0 * rhs.0 .1
                    + self.3 .1 * rhs.1 .1
                    + self.3 .2 * rhs.2 .1
                    + self.3 .3 * rhs.3 .1,
                self.3 .0 * rhs.0 .2
                    + self.3 .1 * rhs.1 .2
                    + self.3 .2 * rhs.2 .2
                    + self.3 .3 * rhs.3 .2,
                self.3 .0 * rhs.0 .3
                    + self.3 .1 * rhs.1 .3
                    + self.3 .2 * rhs.2 .3
                    + self.3 .3 * rhs.3 .3,
            ),
        )
    }

    /// Transpose the matrix
    #[inline]
    pub fn transpose(self) -> Self {
        Self(
            Vec4(self.0 .0, self.1 .0, self.2 .0, self.3 .0),
            Vec4(self.0 .1, self.1 .1, self.2 .1, self.3 .1),
            Vec4(self.0 .2, self.1 .2, self.2 .2, self.3 .2),
            Vec4(self.0 .3, self.1 .3, self.2 .3, self.3 .3),
        )
    }

    /// Optimized version of multiplication by a translation matrix.
    #[inline]
    pub fn translate(self, t: Vec3) -> Self {
        Self(
            self.0,
            self.1,
            self.2,
            Vec4(
                self.0 .0 * t.0 + self.1 .0 * t.1 + self.2 .0 * t.2 + self.3 .0,
                self.0 .1 * t.0 + self.1 .1 * t.1 + self.2 .1 * t.2 + self.3 .1,
                self.0 .2 * t.0 + self.1 .2 * t.1 + self.2 .2 * t.2 + self.3 .2,
                self.0 .3 * t.0 + self.1 .3 * t.1 + self.2 .3 * t.2 + self.3 .3,
            ),
        )
    }

    /// Optimized version of multiplication by an x-rotation matrix.
    #[inline]
    pub fn x_rotate(self, angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            self.0,
            Vec4(
                cos * self.1 .0 + sin * self.2 .0,
                cos * self.1 .1 + sin * self.2 .1,
                cos * self.1 .2 + sin * self.2 .2,
                cos * self.1 .3 + sin * self.2 .3,
            ),
            Vec4(
                cos * self.2 .0 - sin * self.1 .0,
                cos * self.2 .1 - sin * self.1 .1,
                cos * self.2 .2 - sin * self.1 .2,
                cos * self.2 .3 - sin * self.1 .3,
            ),
            self.3,
        )
    }

    /// Optimized version of multiplication by an y-rotation matrix.
    #[inline]
    pub fn y_rotate(self, angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            Vec4(
                cos * self.0 .0 - sin * self.2 .0,
                cos * self.0 .1 - sin * self.2 .1,
                cos * self.0 .2 - sin * self.2 .2,
                cos * self.0 .3 - sin * self.2 .3,
            ),
            self.1,
            Vec4(
                sin * self.0 .0 + cos * self.2 .0,
                sin * self.0 .1 + cos * self.2 .1,
                sin * self.0 .2 + cos * self.2 .2,
                sin * self.0 .3 + cos * self.2 .3,
            ),
            self.3,
        )
    }

    /// Optimized version of multiplication by an z-rotation matrix.
    #[inline]
    pub fn z_rotate(self, angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self(
            Vec4(
                cos * self.0.0 + sin * self.1.0,
                cos * self.0.1 + sin * self.1.1,
                cos * self.0.2 + sin * self.1.2,
                cos * self.0.3 + sin * self.1.3,
            ),
            Vec4(
                cos * self.1.0 - sin * self.0.0,
                cos * self.1.1 - sin * self.0.1,
                cos * self.1.2 - sin * self.0.2,
                cos * self.1.3 - sin * self.0.3,
            ),
            self.2,
            self.3,
        )
    }

    /// Optimized version of multiplication by an axis-rotation matrix.
    #[inline]
    pub fn axis_rotate(self, axis: Vec3, angle: f32) -> Self {
        let Vec3 (x, y, z) = axis;
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let sin = angle.sin();
        let cos = angle.cos();
        let p = 1.0 - cos;

        let r = Mat4(Vec4(
                xx + (1.0 - xx) * cos,
                x * y * p + z * sin,
                x * z * p - y * sin,
                0.0,
            ), Vec4(
                x * y * p - z * sin,
                yy + (1.0 - yy) * cos,
                y * z * p + x * sin,
                0.0,
            ), Vec4(
                x * z * p + y * sin,
                y * z * p - x * sin,
                zz + (1.0 - zz) * cos,
                0.0,
            ), Vec4(0.0, 0.0, 0.0, 0.0),
        );

        Self(
            Vec4(
                r.0.0 * self.0.0 + r.0.1 * self.1.0 + r.0.2 * self.2.0,
                r.0.0 * self.0.1 + r.0.1 * self.1.1 + r.0.2 * self.2.1,
                r.0.0 * self.0.2 + r.0.1 * self.1.2 + r.0.2 * self.2.2,
                r.0.0 * self.0.3 + r.0.1 * self.1.3 + r.0.2 * self.2.3,
            ),
            Vec4(
                r.1.0 * self.0.0 + r.1.1 * self.1.0 + r.1.2 * self.2.0,
                r.1.0 * self.0.1 + r.1.1 * self.1.1 + r.1.2 * self.2.1,
                r.1.0 * self.0.2 + r.1.1 * self.1.2 + r.1.2 * self.2.2,
                r.1.0 * self.0.3 + r.1.1 * self.1.3 + r.1.2 * self.2.3,
            ),
            Vec4(
                r.2.0 * self.0.0 + r.2.1 * self.1.0 + r.2.2 * self.2.0,
                r.2.0 * self.0.1 + r.2.1 * self.1.1 + r.2.2 * self.2.1,
                r.2.0 * self.0.2 + r.2.1 * self.1.2 + r.2.2 * self.2.2,
                r.2.0 * self.0.3 + r.2.1 * self.1.3 + r.2.2 * self.2.3,
            ),
            self.3,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Mat4, Vec3, Vec4};
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};
    use std::fmt::{Debug, Formatter, Result};

    impl Debug for Mat4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Mat4({:?}, {:?}, {:?}, {:?})",
                self.0, self.1, self.2, self.3
            )
        }
    }

    #[test]
    fn equal() {
        let a = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        let b = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn copy() {
        let a = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn identity() {
        let a = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(Mat4::identity(), a);
    }

    #[test]
    fn translation() {
        let a = Mat4::translation(Vec3(1.0, 1.0, 1.0));
        let b = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(1.0, 1.0, 1.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn look_at() {
        let a = Mat4::look_at(
            Vec3(2.3, 2.3, 6.5),
            Vec3(0.0, 0.0, 0.0),
            Vec3(0.0, 0.0, 1.0),
        );
        let b = Mat4(
            Vec4(-0.7071, 0.7071, 0.0, 0.0),
            Vec4(-0.6323, -0.6323, 0.4475, 0.0),
            Vec4(0.3164, 0.3164, 0.8942, 0.0),
            Vec4(2.3, 2.3, 6.5, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn perspective() {
        let a = Mat4::perspective(FRAC_PI_2, 1.7778, 0.01, 100.0);
        let b = Mat4(
            Vec4(0.5625, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, -1.0002, -1.0),
            Vec4(0.0, 0.0, -0.02, 0.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn orthographic() {
        let a = Mat4::orthographic(0.0, 1.0, 0.0, 1.0, 0.01, 100.0);
        let b = Mat4(
            Vec4(2.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 2.0, 0.0, 0.0),
            Vec4(0.0, 0.0, -0.02, 0.0),
            Vec4(-1.0, -1.0, -1.0002, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn frustum() {
        let a = Mat4::frustum(0.0, 1.0, 0.0, 1.0, 0.01, 100.0);
        let b = Mat4(
            Vec4(0.02, 0.0, 0.0, 0.0),
            Vec4(0.0, 0.02, 0.0, 0.0),
            Vec4(1.0, 1.0, -1.0002, -1.0),
            Vec4(0.0, 0.0, -0.02, 0.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn x_rotation() {
        let a = Mat4::x_rotation(FRAC_PI_3);
        let b = Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 0.5, 0.866, 0.0),
            Vec4(0.0, -0.866, 0.5, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn y_rotation() {
        let a = Mat4::y_rotation(FRAC_PI_3);
        let b = Mat4(
            Vec4(0.5, 0.0, -0.866, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.866, 0.0, 0.5, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn z_rotation() {
        let a = Mat4::z_rotation(FRAC_PI_3);
        let b = Mat4(
            Vec4(0.5, 0.866, 0.0, 0.0),
            Vec4(-0.866, 0.5, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn axis_rotation() {
        let a = Mat4::axis_rotation(Vec3(0.4570, 0.8437, 0.2812), FRAC_PI_3);
        let b = Mat4(
            Vec4(0.6044, 0.4364, -0.6664, 0.0),
            Vec4(-0.0507, 0.8559, 0.5144, 0.0),
            Vec4(0.7950, -0.2771, 0.5395, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn dot_product() {
        let a = Mat4(
            Vec4(2.0, 4.0, 8.0, 16.0),
            Vec4(16.0, 2.0, 4.0, 8.0),
            Vec4(8.0, 16.0, 2.0, 4.0),
            Vec4(4.0, 8.0, 16.0, 2.0),
        );
        let b = Mat4(
            Vec4(1.0, 2.0, 3.0, 4.0),
            Vec4(0.0, 1.0, 2.0, 3.0),
            Vec4(0.0, 0.0, 1.0, 2.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        );
        let c = Mat4(
            Vec4(2.0, 8.0, 22.0, 52.0),
            Vec4(16.0, 34.0, 56.0, 86.0),
            Vec4(8.0, 32.0, 58.0, 88.0),
            Vec4(4.0, 16.0, 44.0, 74.0),
        );
        assert_eq!(c, a.dot(b));
    }

    #[test]
    fn transpose() {
        let a = Mat4(
            Vec4(2.0, 4.0, 8.0, 16.0),
            Vec4(32.0, 64.0, 128.0, 256.0),
            Vec4(512.0, 1024.0, 2048.0, 4096.0),
            Vec4(8192.0, 16384.0, 32768.0, 65536.0),
        );
        let b = Mat4(
            Vec4(2.0, 32.0, 512.0, 8192.0),
            Vec4(4.0, 64.0, 1024.0, 16384.0),
            Vec4(8.0, 128.0, 2048.0, 32768.0),
            Vec4(16.0, 256.0, 4096.0, 65536.0),
        );
        assert_eq!(b, a.transpose());
    }

    #[test]
    fn translate() {
        let a = Mat4(
            Vec4(2.0, 4.0, 8.0, 16.0),
            Vec4(16.0, 2.0, 4.0, 8.0),
            Vec4(8.0, 16.0, 2.0, 4.0),
            Vec4(4.0, 8.0, 16.0, 2.0),
        );
        let b = Mat4(
            Vec4(2.0, 4.0, 8.0, 16.0),
            Vec4(16.0, 2.0, 4.0, 8.0),
            Vec4(8.0, 16.0, 2.0, 4.0),
            Vec4(80.6, 53.1999, 52.3999, 74.7999),
        );
        assert_eq!(a.translate(Vec3(2.3, 3.6, 1.8)), b);

        // The same result as BA, where B is a translation matrix.
        assert_eq!(Mat4::translation(Vec3(2.3, 3.6, 1.8)).dot(a), b);
    }

    #[test]
    fn x_rotate() {
        let a = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let b = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(0.0495, 0.6117, 1.3526, -1.5193),
            Vec4(1.4142, -0.3396, -0.1428, -3.6484),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        assert_eq!(a.x_rotate(FRAC_PI_3), b);

        // The same result as BA, where B is a x-rotation matrix.
        assert_eq!(Mat4::x_rotation(FRAC_PI_3).dot(a), b);
    }

    #[test]
    fn y_rotate() {
        let a = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let b = Mat4(
            Vec4(-0.1495, 0.3382, -0.1526, 3.7193),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(1.2410, 1.3058, 1.9356, 0.1620),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        assert_eq!(a.y_rotate(FRAC_PI_3), b);

        // The same result as BA, where B is a y-rotation matrix.
        assert_eq!(Mat4::y_rotation(FRAC_PI_3).dot(a), b);
    }

    #[test]
    fn z_rotate() {
        let a = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let b = Mat4(
            Vec4(-0.5392, 1.1696, 1.4928, 3.0784),
            Vec4(-1.466, -0.8258, -0.9856, -0.532),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        assert_eq!(a.z_rotate(FRAC_PI_3), b);

        // The same result as BA, where B is an z-rotation matrix.
        assert_eq!(Mat4::z_rotation(FRAC_PI_3).dot(a), b);
    }

    #[test]
    fn axis_rotate() {
        let a = Mat4(
            Vec4(1.0, 1.3, 1.6, 2.0),
            Vec4(-1.2, 0.6, 0.8, 2.4),
            Vec4(0.75, 0.36, 1.1, -3.14),
            Vec4(0.87, 0.32, 1.1, -2.0),
        );
        let b = Mat4(
            Vec4(-0.41895834, 0.8076307, 0.58307594, 4.3485293),
            Vec4(-0.6920404, 0.632769, 1.1693841, 0.3375045),
            Vec4(1.5321517, 1.0613393, 1.6436427, -0.76946425),
            Vec4(0.87, 0.32, 1.1, -2.0)
        );
        assert_eq!(a.axis_rotate(Vec3(0.457, 0.8437, 0.2812), FRAC_PI_3), b);

        // The same result as BA, where B is an axis-rotation matrix.
        assert_eq!(b, Mat4::axis_rotation(Vec3(0.457, 0.8437, 0.2812), FRAC_PI_3).dot(a));
    }
}
