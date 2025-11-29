use core::mem::MaybeUninit;

/// Lazily initialized value.
///
/// Wraps `T` inside `MaybeUninit<T>` and tracks whether it has been
/// initialized.
#[derive(Clone, Copy)]
pub struct Lazy<T: Clone + Copy> {
    init: bool,
    pub inner: MaybeUninit<T>,
}

impl<T: Clone + Copy> Default for Lazy<T> {
    fn default() -> Self {
        Self {
            init: false,
            inner: MaybeUninit::uninit(),
        }
    }
}

impl<T: Clone + Copy> Lazy<T> {
    /// Update the inner value
    ///
    /// If uninitialized, run `init_fn` to initialize the value.
    /// If initialized, give mutable access for update.
    #[inline]
    pub fn update(
        &mut self,
        init_fn: impl FnOnce() -> T,
        update_fn: impl FnOnce(&mut T) -> Option<()>,
    ) -> Option<()> {
        if !self.init {
            self.init = true;
            self.inner = MaybeUninit::new(init_fn());
            Some(())
        } else {
            let inner = unsafe { self.inner.assume_init_mut() };
            update_fn(inner)
        }
    }
}
