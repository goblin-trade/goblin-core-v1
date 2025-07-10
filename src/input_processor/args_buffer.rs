///! We read calldata into a buffer of size INPUT_SIZE.
///! The actual size of calldata equals to `len`. Ensure that it is less than or equal to `INPUT_SIZE`.

pub const INPUT_SIZE: usize = 512;
pub type ArgsBuffer = [u8; INPUT_SIZE];

pub trait ArgsDecoder {
    fn decode_ref<T>(&self, start: usize) -> &T;
    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T];
}

impl ArgsDecoder for ArgsBuffer {
    ///! Zero-copy decoding of a reference to a value of type `T` from the buffer.
    fn decode_ref<T>(&self, start: usize) -> &T {
        let end = start + core::mem::size_of::<T>();
        unsafe { &*(self[start..end].as_ptr() as *const T) }
    }

    ///! Zero-copy decoding of a slice of values of type `T` from the buffer.
    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T] {
        let byte_len = len * core::mem::size_of::<T>();
        let end = start + byte_len;

        unsafe { core::slice::from_raw_parts(self[start..end].as_ptr() as *const T, len) }
    }
}
