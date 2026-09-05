#[cfg(all(not(any(test, feature = "sdk")), target_arch = "wasm32"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    crate::hostio::hostio_unsafe::pay_for_memory_grow(0);
    panic!();
}
