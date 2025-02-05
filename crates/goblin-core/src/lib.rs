#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
