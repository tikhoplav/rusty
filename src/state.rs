use crate::color::Color;
use crate::vec3::Vec3;

#[repr(C)]
pub struct Vertex(pub Vec3, pub Color);

#[repr(C)]
pub struct State {
    pub vertices: Vec<Vertex>,
    pub count: usize,
}
