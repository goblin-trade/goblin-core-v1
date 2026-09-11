#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub extern "C" fn mark_used() {
    unsafe { goblin_hostio::hostio_unsafe::pay_for_memory_grow(0) };
    panic!();
}
