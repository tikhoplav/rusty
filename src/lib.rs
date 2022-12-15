#[no_mangle]
pub extern fn greet() -> *mut String {
    let s = String::from("Hello, World!!!");
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub extern fn motd() -> *mut String {
    let s = String::from(r#"
        Have a nice day, developer!

        My name is Rusty and I'm here to bring you joi.
        I will help you on your way of bringing the best
        game to live.

        Be patient and everything desired will come true.
    "#);
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub extern fn gen() -> *mut Vec<i32> {
    let v: Vec<i32> = vec![5092, 64, 255, 23, 360, 11392];
    Box::into_raw(Box::new(v))
}

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
