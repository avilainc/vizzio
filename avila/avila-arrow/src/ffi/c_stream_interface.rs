//! Arrow C Stream Interface: ArrowArrayStream

use std::ffi::c_void;
use super::c_data_interface::{ArrowArray, ArrowSchema};

/// ArrowArrayStream C struct
#[repr(C)]
pub struct ArrowArrayStream {
    pub get_schema: Option<
        unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowSchema) -> i32
    >,
    pub get_next: Option<
        unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowArray) -> i32
    >,
    pub get_last_error: Option<
        unsafe extern "C" fn(*mut ArrowArrayStream) -> *const i8
    >,
    pub release: Option<unsafe extern "C" fn(*mut ArrowArrayStream)>,
    pub private_data: *mut c_void,
}

impl ArrowArrayStream {
    pub fn empty() -> Self {
        Self {
            get_schema: None,
            get_next: None,
            get_last_error: None,
            release: None,
            private_data: std::ptr::null_mut(),
        }
    }
}
