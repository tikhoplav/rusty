pub mod core;
pub mod state;

use crate::core::{Color, Vec3, Vec4, Mat4};
use crate::state::{State, Vertex};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};
use std::sync::Mutex;

// State is stored as a global variable, since it has to be used each update
// without a reference passed over FFI.
static STATE: Mutex<State> = Mutex::new(State {
    view_matrix: Mat4(
        Vec4(1.0, 0.0, 0.0, 0.0),
        Vec4(0.0, 1.0, 0.0, 0.0),
        Vec4(0.0, 0.0, 1.0, 0.0),
        Vec4(0.0, 0.0, 0.0, 1.0),
    ),
    vertices: Vec::new(),
    count: 0,
});

#[no_mangle]
pub extern "C" fn set_aspect(aspect: f32) {
    let state = &mut *STATE.lock().unwrap();
    state.view_matrix = (
        Mat4::perspective(FRAC_PI_3 * 2.0, 1.0 / aspect, 0.01, 100.0)
        * Mat4::look_at(
            Vec3(3.0, -3.0, 9.0),
            Vec3(0.0, 0.0, 0.0),
            Vec3(0.0, 0.0, 1.0)
        )
    )
        .scale(Vec3(0.05, 0.05, 0.05))
        .inverse();
}

#[no_mangle]
pub extern "C" fn state_view_data() -> *const Mat4 {
    &(&*STATE.lock().unwrap()).view_matrix
}

#[no_mangle]
pub extern "C" fn state_data() -> *const Vertex {
    let v = &(&*STATE.lock().unwrap()).vertices;
    <Vec<Vertex> as AsRef<Vec<Vertex>>>::as_ref(v).as_ptr()
}

#[no_mangle]
pub extern "C" fn state_len() -> usize {
    let v = &(&*STATE.lock().unwrap()).vertices;
    <Vec<Vertex> as AsRef<Vec<Vertex>>>::as_ref(v).len()
}

#[no_mangle]
pub extern "C" fn gen() {
    let state = &mut *STATE.lock().unwrap();

    state.vertices.push(Vertex(
        Vec3(0.0, 0.866025404, 0.0),
        Color(255, 0, 0, 255)
    ));

    state.vertices.push(Vertex(
        Vec3(1.0, -0.866025404, 0.0),
        Color(0, 255, 0, 255),
    ));

    state.vertices.push(Vertex(
        Vec3(-1.0, -0.866025404, 0.0),
        Color(0, 0, 255, 255),
    ));
}

#[no_mangle]
pub extern "C" fn update() {
    let state = &mut *STATE.lock().unwrap();
    let v = &mut state.vertices;

    for (i, vertex) in v.iter_mut().enumerate() {
        vertex.1 = Vec4(
            127.0 - 127.0 * ((state.count as f32) / 120.0 + (i as f32) * FRAC_PI_3).cos() as f32,
            127.0
                - 127.0
                    * ((state.count as f32) / 60.0 + FRAC_PI_2 + (i as f32) * FRAC_PI_3).sin()
                        as f32,
            127.0 - 127.0 * ((state.count as f32) / 120.0 + (i as f32) * FRAC_PI_3).sin() as f32,
            255.0,
        )
        .into();
    }

    state.count += 1;
}
