#![allow(unused_imports, unused_variables)]

use std::ffi::{c_char, CString};

#[no_mangle]
pub fn get_string() -> *const u8 {
    // let string = CString::new("hello, from rust world").unwrap();
    b"Hello from rust-world\0".as_ptr()
}
