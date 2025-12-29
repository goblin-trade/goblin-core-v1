use crate::input_processor::ArgsBuffer;
use core::cell::Cell;

pub struct DecodeCtx<'a> {
    pub args: &'a [u8],
    pub offset: Cell<usize>,
}

impl<'a> DecodeCtx<'a> {
    pub fn new(args_buffer: &'a ArgsBuffer, len: usize) -> Self {
        Self {
            args: &args_buffer[..len],
            offset: Cell::<usize>::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.args.len()
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
