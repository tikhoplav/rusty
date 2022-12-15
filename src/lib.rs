use std::sync::Mutex;

#[no_mangle]
pub unsafe extern fn vec_data(v: *mut Vec<u8>) -> *const u8 {
    v.as_ref().expect("Not available").as_ptr()
}

#[no_mangle]
pub unsafe extern fn vec_len(v: *mut Vec<u8>) -> usize {
    v.as_ref().expect("Not available").len()
}

#[no_mangle]
pub unsafe extern fn destroy(s: *mut Vec<u8>) {
    drop(Box::from_raw(s))
}

#[no_mangle]
pub extern fn greet() -> *mut String {
    let s = String::from(r#"
    Have a nice day, developer!

    My name is Rusty and I'm here to bring you joi.
    I will help you on your way of bringing the best
    game to live.

    Be patient and everything desired will come true.
"#);
    Box::into_raw(Box::new(s))
}

#[repr(C)]
pub struct Vector(f32, f32);

#[repr(C)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn rotate(&mut self, a: Color) {
        self.0 = ((self.0 as u32 + a.0 as u32) % 256) as u8;
        self.1 = ((self.1 as u32 + a.1 as u32) % 256) as u8;
        self.2 = ((self.2 as u32 + a.2 as u32) % 256) as u8;
        self.3 = ((self.3 as u32 + a.3 as u32) % 256) as u8;
    }
}

#[repr(C)]
pub struct Vertex(Vector, Color);

// State is stored as a global variable, since it has to be used each update
// without a reference passed over FFI.
static STATE: Mutex<Vec<Vertex>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern fn gen() {
    let v = &mut *STATE.lock().unwrap();
    
    v.push(Vertex (
        Vector (0.0, 230.940107676),
        Color (200, 0, 0, 255),
    ));

    v.push(Vertex (
        Vector (200.0, -115.470053838),
        Color (0, 200, 0, 255),
    ));

    v.push(Vertex (
        Vector (-200.0, -115.470053838),
        Color (0, 0, 200, 255),
    ));
}

#[no_mangle]
pub extern fn state_data() -> *const Vertex {
    let v: &Vec<Vertex> = &*STATE.lock().unwrap();
    <Vec<Vertex> as AsRef<Vec<Vertex>>>::as_ref(v).as_ptr()
}

#[no_mangle]
pub extern fn state_len() -> usize {
    let v: &Vec<Vertex> = &*STATE.lock().unwrap();
    <Vec<Vertex> as AsRef<Vec<Vertex>>>::as_ref(v).len()
}

#[no_mangle]
pub extern fn update() {
    let v = &mut *STATE.lock().unwrap();
    for (i, vertex) in v.iter_mut().enumerate() {
        let c = match i {
            0 => Color(1, 0, 0, 0),
            1 => Color(0, 1, 0, 0),
            2 => Color(0, 0, 1, 0),
            _ => Color(0, 0, 0, 0),
        };
        vertex.1.rotate(c);
    }
}