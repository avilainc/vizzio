//! C API for avila-arrow

#[no_mangle]
pub extern "C" fn avila_arrow_version() -> *const i8 {
    "0.1.0\0".as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn avila_arrow_init() -> i32 {
    0 // Success
}
