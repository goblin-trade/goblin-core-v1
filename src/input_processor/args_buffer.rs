use crate::input_processor::ArgsDecoder;

pub const INPUT_SIZE: usize = 512;
pub type ArgsBuffer = [u8; INPUT_SIZE];

impl ArgsDecoder for ArgsBuffer {
    fn decode_ref<T>(&self, start: usize) -> &T {
        let end = start + core::mem::size_of::<T>();
        unsafe { &*(self[start..end].as_ptr() as *const T) }
    }

    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T] {
        let byte_len = len * core::mem::size_of::<T>();
        let end = start + byte_len;

        unsafe { core::slice::from_raw_parts(self[start..end].as_ptr() as *const T, len) }
    }
}
