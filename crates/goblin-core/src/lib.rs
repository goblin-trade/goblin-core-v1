#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
use alloc::vec::Vec;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

extern "C" {
    fn read_args(dest: *mut u8);
    fn write_result(data: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    let mut input = Vec::<u8>::with_capacity(len);

    unsafe {
        // Read the input data
        read_args(input.as_mut_ptr());
    }

    // Convert the length to bytes
    let result = len.to_le_bytes();

    unsafe {
        // Write the length back as result
        write_result(result.as_ptr(), result.len());
    }

    0
}
