use crate::core::{Color, Mat4, Vec3};

#[repr(C)]
pub struct Screen(pub f32, pub f32);

#[repr(C)]
pub struct Vertex(pub Vec3, pub Color);

#[repr(C)]
pub struct State {
    pub screen: Screen,
    pub camera: Vec3,
    pub view_matrix: Mat4,
    pub vertices: Vec<Vertex>,
    pub count: usize,
}
