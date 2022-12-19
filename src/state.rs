use crate::color::Color;
use crate::vec4::Vec4;

#[repr(C)]
pub struct Vertex(pub Vec4, pub Color);

#[repr(C)]
pub struct State {
    pub vertices: Vec<Vertex>,
    pub count: usize,
}
