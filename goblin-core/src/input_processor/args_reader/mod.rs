pub mod args_buffer;
pub mod traits;

pub use args_buffer::*;
use goblin_hostio::hostio_unsafe;
pub use traits::*;

use core::cell::Cell;
use core::mem::MaybeUninit;

pub struct ArgsReader {
    pub args: ArgsBuffer,
    pub len: usize,
    pub offset: Cell<usize>,
}

impl ArgsReader {
    pub fn new(len: usize) -> Self {
        let mut args_buffer = MaybeUninit::<ArgsBuffer>::uninit();
        let args = unsafe {
            hostio_unsafe::read_args(args_buffer.as_mut_ptr() as *mut u8);
            args_buffer.assume_init()
        };

        Self {
            args,
            len,
            offset: Cell::<usize>::default(),
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut args = [0u8; crate::input_processor::INPUT_SIZE];
        let len = slice.len().min(crate::input_processor::INPUT_SIZE);
        args[..len].copy_from_slice(&slice[..len]);
        Self {
            args,
            len: slice.len(),
            offset: Cell::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn advance_offset(&self, increment: usize) {
        let current_offset = self.offset.get();
        self.offset.set(current_offset + increment);
    }

    // Zero copy decode
    pub fn zero_copy_unchecked<T>(&self) -> &T {
        let start = self.offset.get();
        let end = start + core::mem::size_of::<T>();
        self.offset.set(end);

        unsafe { &*(self.args[start..end].as_ptr() as *const T) }
    }

    pub fn zero_copy_slice_unchecked<T>(&self, slice_len: usize) -> &[T] {
        let start = self.offset.get();
        let end = start + slice_len * core::mem::size_of::<T>();
        self.offset.set(end);

        unsafe {
            core::slice::from_raw_parts(self.args[start..end].as_ptr() as *const T, slice_len)
        }
    }
}
