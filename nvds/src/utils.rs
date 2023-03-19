pub fn version() -> (u32, u32) {
    unsafe {
        let mut major: u32 = 0;
        let mut minor: u32 = 0;
        nvidia_deepstream_sys::nvds_version(&mut major, &mut minor);
        (major, minor)
    }
}

pub fn print_version() {
    unsafe { nvidia_deepstream_sys::nvds_version_print() }
}

pub fn print_dependencies_version() {
    unsafe { nvidia_deepstream_sys::nvds_dependencies_version_print() }
}
