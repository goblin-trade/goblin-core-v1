use core::mem::MaybeUninit;

use crate::{goblin_error::GoblinError, hostio, require};

pub struct CallPayload<'a> {
    pub len: usize,
    pub offset: usize,
    pub input: &'a [u8; 512],
}

impl<'a> CallPayload<'a> {
    pub fn new(
        len: usize,
        input_maybe: &'a mut MaybeUninit<[u8; 512]>,
    ) -> Result<Self, GoblinError> {
        let offset = 4;
        require!(len >= offset, GoblinError::InvalidPayload);

        let input = unsafe {
            hostio::read_args(input_maybe.as_mut_ptr() as *mut u8);
            input_maybe.assume_init_ref()
        };

        Ok(Self { len, offset, input })
    }

    pub fn decode_ref<T>(&mut self) -> Result<&T, GoblinError> {
        let start_index = self.offset;
        self.offset += core::mem::size_of::<T>();
        require!(self.len >= self.offset, GoblinError::InvalidPayload);

        let result = unsafe { &*(self.input[start_index..self.offset].as_ptr() as *const T) };
        Ok(result)
    }

    pub fn decode<T: Clone>(&mut self) -> Result<T, GoblinError> {
        let result_ref = self.decode_ref::<T>()?;
        Ok(result_ref.clone())
    }

    pub fn decode_slice<T>(&mut self, len: usize) -> Result<&[T], GoblinError> {
        let start_index = self.offset;
        self.offset += core::mem::size_of::<T>() * len;
        require!(self.len >= self.offset, GoblinError::InvalidPayload);

        let result = unsafe {
            core::slice::from_raw_parts(
                self.input[start_index..self.offset].as_ptr() as *const T,
                len,
            )
        };

        Ok(result)
    }
}
