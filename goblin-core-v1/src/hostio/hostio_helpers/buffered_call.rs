use core::mem::MaybeUninit;

/// Helper function to build MaybeUninit buffer, perform hostio call `f()`
/// and return the unwrapped value
///
/// MaybeUninit saves gas by avoiding unnecessary zero-fill
pub unsafe fn buffered_call<K>(f: impl FnOnce(*mut u8)) -> K {
    let mut buffer = MaybeUninit::<K>::uninit();
    f(buffer.as_mut_ptr() as *mut u8);
    buffer.assume_init()
}
