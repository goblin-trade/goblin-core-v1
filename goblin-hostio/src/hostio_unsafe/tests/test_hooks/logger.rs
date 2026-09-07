#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_i64(value: i64) {
    println!("i64({})", value);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_txt(text: *const u8, len: usize) {
    let slice = unsafe { core::slice::from_raw_parts(text, len) };
    if let Ok(text) = core::str::from_utf8(slice) {
        println!("Stylus says: {}", text);
    }
}
