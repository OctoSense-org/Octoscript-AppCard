//! hilog, tag `camera-oh` (read with `hilog -x | grep camera-oh`).
use std::ffi::CString;

extern "C" {
    fn OH_LOG_Print(log_type: i32, level: i32, domain: u32, tag: *const std::os::raw::c_char, fmt: *const std::os::raw::c_char, ...) -> i32;
}

fn print(level: i32, msg: &str) {
    let Ok(c) = CString::new(msg) else { return };
    unsafe { OH_LOG_Print(0, level, 0xCA11, c"camera-oh".as_ptr(), c"%{public}s".as_ptr(), c.as_ptr()) };
}
pub fn info(msg: &str) { print(4, msg); }
pub fn error(msg: &str) { print(6, msg); }

