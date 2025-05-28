use core::mem::MaybeUninit;

/// A wrapper for creating and initializing an uninitialized buffer via a hostio call.
pub struct HostioBuffer<T> {
    inner: MaybeUninit<T>,
}

impl<T> HostioBuffer<T> {
    /// Create a new buffer and let the provided function populate it.
    ///
    /// # Safety
    /// The provided function must fully initialize all bytes of `T`.
    pub fn new(f: impl FnOnce(*mut u8)) -> Self {
        let mut buffer = MaybeUninit::<T>::uninit();
        f(buffer.as_mut_ptr() as *mut u8);
        Self { inner: buffer }
    }

    /// Get a reference to the initialized buffer.
    ///
    /// # Safety
    /// Caller must ensure the buffer is fully initialized (as guaranteed by the hostio call).
    pub fn as_ref(&self) -> &T {
        unsafe { self.inner.assume_init_ref() }
    }
}
