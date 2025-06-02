use crate::{hostio::hostio_read_args, hostio_buffer::HostioBuffer};

pub const INPUT_SIZE: usize = 512;
pub type PayloadBuffer = [u8; INPUT_SIZE];

pub struct CallPayload {
    pub len: usize,
    pub input: HostioBuffer<PayloadBuffer>,
}

impl CallPayload {
    pub fn new(len: usize) -> Self {
        let input = unsafe { hostio_read_args() };
        Self { len, input }
    }

    pub fn decode_ref<T>(&self, start: usize) -> &T {
        let end = start + core::mem::size_of::<T>();
        unsafe { &*(self.input.as_ref()[start..end].as_ptr() as *const T) }
    }

    pub fn decode_slice<T>(&self, start: usize, len: usize) -> &[T] {
        let byte_len = len * core::mem::size_of::<T>();
        let end = start + byte_len;

        unsafe {
            core::slice::from_raw_parts(self.input.as_ref()[start..end].as_ptr() as *const T, len)
        }
    }
}
