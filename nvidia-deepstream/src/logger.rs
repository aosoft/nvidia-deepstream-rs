use std::ffi::CStr;

pub fn log_open() {
    unsafe { nvidia_deepstream_sys::nvds_log_open(); }
}

pub fn log_close() {
    unsafe { nvidia_deepstream_sys::nvds_log_close(); }
}

pub fn log(category: &CStr, priority: i32, data: &CStr) {
    unsafe {
        nvidia_deepstream_sys::nvds_log(category.as_ptr(), priority, data.as_ptr());
    }
}