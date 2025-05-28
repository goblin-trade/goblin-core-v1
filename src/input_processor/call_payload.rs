use core::mem::MaybeUninit;

use crate::{goblin_error::GoblinError, hostio, require};

pub const INPUT_SIZE: usize = 512;

pub struct CallPayload {
    pub len: usize,
    pub offset: usize,
    pub input: MaybeUninit<[u8; INPUT_SIZE]>,
}

impl CallPayload {
    pub fn new(len: usize) -> Self {
        let mut input = MaybeUninit::<[u8; INPUT_SIZE]>::uninit();
        unsafe {
            hostio::read_args(input.as_mut_ptr() as *mut u8);
        };

        Self {
            len,
            offset: 0,
            input,
        }
    }

    pub unsafe fn input_ref(&self) -> &[u8; INPUT_SIZE] {
        self.input.assume_init_ref()
    }

    pub fn advance_offset<T>(&mut self) -> Result<(), GoblinError> {
        self.offset += core::mem::size_of::<T>();
        require!(self.len >= self.offset, GoblinError::InvalidPayload);
        Ok(())
    }

    pub fn decode_ref<T>(&mut self) -> Result<&T, GoblinError> {
        let start_index = self.offset;
        self.advance_offset::<T>()?;
        let result = unsafe { &*(self.input_ref()[start_index..self.offset].as_ptr() as *const T) };
        Ok(result)
    }

    pub fn decode<T: Clone>(&mut self) -> Result<T, GoblinError> {
        let result_ref = self.decode_ref::<T>()?;
        Ok(result_ref.clone())
    }

    pub fn decode_slice<T>(&mut self, len: usize) -> Result<&[T], GoblinError> {
        let start_index = self.offset;
        self.advance_offset::<T>()?;

        let result = unsafe {
            core::slice::from_raw_parts(
                self.input_ref()[start_index..self.offset].as_ptr() as *const T,
                len,
            )
        };

        Ok(result)
    }
}
