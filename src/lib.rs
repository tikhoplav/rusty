pub mod approx;
pub mod color;
pub mod mat4;
pub mod vec3;
pub mod vec4;
pub mod state;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};
use crate::color::Color;
use crate::state::{State, Vertex};
use crate::vec3::Vec3;
use crate::vec4::Vec4;
use std::sync::Mutex;

// State is stored as a global variable, since it has to be used each update
// without a reference passed over FFI.
static STATE: Mutex<State> = Mutex::new(State {
    vertices: Vec::new(),
    count: 0,
});

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
        Vec3(0.0, 346.410161514, 0.0),
        Color(255, 0, 0, 255),
    ));

    state.vertices.push(Vertex(
        Vec3(400.0, -346.410161514, 0.0),
        Color(0, 255, 0, 255),
    ));

    state.vertices.push(Vertex(
        Vec3(-400.0, -346.410161514, 0.0),
        Color(0, 0, 255, 255),
    ));
}

#[no_mangle]
pub extern "C" fn update() {
    let state = &mut *STATE.lock().unwrap();
    let v = &mut state.vertices;

    for (i, vertex) in v.iter_mut().enumerate() {
        vertex.1 = Vec4(
            127.0 - 127.0 * ((state.count as f64) / 120.0 + (i as f64) * FRAC_PI_3).cos() as f32,
            127.0 - 127.0 * ((state.count as f64) / 60.0 + FRAC_PI_2 + (i as f64) * FRAC_PI_3).sin() as f32,
            127.0 - 127.0 * ((state.count as f64) / 120.0 + (i as f64) * FRAC_PI_3).sin() as f32,
            255.0
        ).into();
    }

    state.count += 1;
}
