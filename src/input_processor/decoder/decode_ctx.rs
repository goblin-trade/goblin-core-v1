use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, DecodePrimitive},
};
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

    pub fn decode<T: DecodePrimitive>(&self) -> Result<T, GoblinError> {
        let offset = self.offset.get();
        let size = core::mem::size_of::<T>();
        crate::require!(
            self.len() >= offset + size,
            crate::goblin_error::GoblinError::InvalidPayload
        );

        let value = Self::decode_unchecked_no_advance::<T>(self);
        self.offset.set(offset + size);

        Ok(value)
    }

    // TODO use in place of decode_ref_unchecked where we still need to advance offset
    pub fn decode_unchecked<T: DecodePrimitive>(&self) -> T {
        let offset = self.offset.get();
        let size = core::mem::size_of::<T>();
        let value = Self::decode_unchecked_no_advance::<T>(self);
        self.offset.set(offset + size);

        value
    }

    // TODO improvements
    //
    // - This is just a proxy to DecodePrimitive. Can we replace with common Decodable trait?
    // - Does not advance offset unlike the zero copy versions
    pub fn decode_unchecked_no_advance<T: DecodePrimitive>(&self) -> T {
        T::from_le_bytes_at(self.args, self.offset.get())
    }
}
