use core::mem::MaybeUninit;

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
    pub fn init(&self) -> bool {
        self.init
    }

    pub fn new(val: T) -> Self {
        Self {
            init: true,
            inner: MaybeUninit::new(val),
        }
    }
}
