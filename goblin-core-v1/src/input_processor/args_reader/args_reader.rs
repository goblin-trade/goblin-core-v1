use crate::hostio::hostio_unsafe;
use crate::input_processor::ArgsBuffer;
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

    pub fn len(&self) -> usize {
        self.len
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
