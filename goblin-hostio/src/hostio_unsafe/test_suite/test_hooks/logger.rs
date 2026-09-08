pub fn log_i64(value: i64) {
    println!("i64({})", value);
}

/// Log text into the debug logger
///
/// # Safety
///
/// Correct length is passed
pub unsafe fn log_txt(text: *const u8, len: usize) {
    let slice = unsafe { core::slice::from_raw_parts(text, len) };
    if let Ok(text) = core::str::from_utf8(slice) {
        println!("Stylus says: {}", text);
    }
}
