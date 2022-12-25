pub mod core;
pub mod state;

use std::f32::consts::{PI, FRAC_PI_3};
use std::sync::Mutex;
use crate::state::{Screen, State, Vertex};
use crate::core::{Color, Mat4, Vec3, Vec4};
use std::mem;

// State is stored as a global variable, since it has to be used each update
// without a reference passed over FFI.
static STATE: Mutex<State> = Mutex::new(State {
    screen: Screen(1980.0, 1024.0),
    camera: Vec3(6.0, -6.0, 12.0),
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
pub extern "C" fn set_screen_width(width: u32) {
    let state = &mut *STATE.lock().unwrap();
    state.screen.0 = width as f32;
}

#[no_mangle]
pub extern "C" fn set_screen_height(height: u32) {
    let state = &mut *STATE.lock().unwrap();
    state.screen.1 = height as f32;
}

#[no_mangle]
pub extern "C" fn rotate_camera(pixels: i32) {
    let state = &mut *STATE.lock().unwrap();
    state.camera = Mat4::z_rotation(PI * (pixels as f32) / state.screen.0) * state.camera;
}

#[no_mangle]
pub extern "C" fn state_view_matrix() -> *const Mat4 {
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
    <Vec<Vertex> as AsRef<Vec<Vertex>>>::as_ref(v).len() * mem::size_of::<Vertex>()
}

#[no_mangle]
pub extern "C" fn gen() {
    let state = &mut *STATE.lock().unwrap();


    state
        .vertices
        .push(Vertex(Vec3(-1.0, -1.0, 0.0), Color(0, 0, 255, 255)));

    state
        .vertices
        .push(Vertex(Vec3(-1.0, 1.0, 0.0), Color(255, 0, 0, 255)));

    state
        .vertices
        .push(Vertex(Vec3(1.0, -1.0, 0.0), Color(0, 255, 0, 255)));

    state
        .vertices
        .push(Vertex(Vec3(1.0, -1.0, 0.0), Color(0, 255, 0, 255)));

    state
        .vertices
        .push(Vertex(Vec3(-1.0, 1.0, 0.0), Color(255, 0, 0, 255)));

    state
        .vertices
        .push(Vertex(Vec3(1.0, 1.0, 0.0), Color(255, 255, 0, 255)));
}

#[no_mangle]
pub extern "C" fn update() {
    let state = &mut *STATE.lock().unwrap();

    state.view_matrix = (Mat4::perspective(
        FRAC_PI_3 * 2.0,
        state.screen.1 / state.screen.0,
        0.1,
        100.0,
    ) * Mat4::look_at(state.camera, Vec3(0.0, 0.0, 1.0), Vec3(0.0, 0.0, 1.0)))
    .scale(Vec3(0.3, 0.3, 0.3))
    .inverse();

    state.count += 1;
}
