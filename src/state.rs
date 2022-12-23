use crate::core::{Color, Vec3, Mat4};

#[repr(C)]
pub struct Vertex(pub Vec3, pub Color);

#[repr(C)]
pub struct State {
    pub view_matrix: Mat4,
    pub vertices: Vec<Vertex>,
    pub count: usize,
}
