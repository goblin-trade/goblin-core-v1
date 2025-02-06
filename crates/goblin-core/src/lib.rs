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

// We need to add vm_hooks otherwise verification will fail
#[link(wasm_import_module = "vm_hooks")]
extern "C" {
    fn read_args(dest: *mut u8);
    fn write_result(data: *const u8, len: usize);
    fn pay_for_memory_grow(pages: u16);
    fn storage_load_bytes32(key: *const u8, dest: *mut u8);
    fn storage_cache_bytes32(key: *const u8, value: *const u8);
    fn storage_flush_cache(clear: bool);
}

#[link(wasm_import_module = "console")]
extern "C" {
    fn log_txt(text: *const u8, len: usize);
}

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    let mut input = Vec::<u8>::with_capacity(len);

    unsafe {
        // Read the input data
        read_args(input.as_mut_ptr());
    }

    let log_message = "Hello world";
    unsafe {
        log_txt(log_message.as_ptr(), log_message.len());
    }

    // Call this at the end to persist values written by storage_cache_bytes32
    unsafe {
        storage_flush_cache(true);
    }

    let result = 2u8.to_le_bytes();
    unsafe {
        // Write the length back as result
        write_result(result.as_ptr(), result.len());
    }

    0
}
