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
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[no_mangle]
pub extern fn gen() -> *mut Vec<Vertex> {
    let v: Vec<Vertex> = vec![
        Vertex {
            x: 0.0,
            y: 230.940107676,
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
        Vertex {
            x: 200.0,
            y: -115.470053838,
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        },
        Vertex {
            x: -200.0,
            y: -115.470053838,
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    ];
    Box::into_raw(Box::new(v))
}
