
// #[no_mangle]
// pub unsafe extern "C" fn vec_data(v: *mut Vec<u8>) -> *const u8 {
//     v.as_ref().expect("Not available").as_ptr()
// }

// #[no_mangle]
// pub unsafe extern "C" fn vec_len(v: *mut Vec<u8>) -> usize {
//     v.as_ref().expect("Not available").len()
// }

// #[no_mangle]
// pub unsafe extern "C" fn destroy(s: *mut Vec<u8>) {
//     drop(Box::from_raw(s))
// }

// #[no_mangle]
// pub extern "C" fn greet() -> *mut String {
//     let s = String::from(
//         r#"
//     Have a nice day, developer!

//     My name is Rusty and I'm here to bring you joi.
//     I will help you on your way of bringing the best
//     game to live.

//     Be patient and everything desired will come true.
// "#,
//     );
//     Box::into_raw(Box::new(s))
// }