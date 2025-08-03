///! We read calldata into a buffer of size INPUT_SIZE.
///! The actual size of calldata equals to `len`. Ensure that it is less than or equal to `INPUT_SIZE`.
///!
///! # Examples
///!
///! ```
///! use goblin_core_v1::input_processor::{ArgsBuffer, ArgsDecoder};
///!
///! let buffer: ArgsBuffer = [0x42, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]; // ... rest filled with zeros
///! let mut offset = 0;
///! let len = 12;
///!
///! // Decode a u32 from the buffer
///! let value: u32 = buffer.decode(&mut offset, len).unwrap(); // Reads 0x00000042 (little-endian)
///!
///! // Decode a u64 from the buffer
///! let value: u64 = buffer.decode(&mut offset, len).unwrap(); // Reads next 8 bytes
///! ```

pub const INPUT_SIZE: usize = 512;
pub type ArgsBuffer = [u8; INPUT_SIZE];

pub trait ArgsDecoder {
    fn decode_ref<T>(&self, start: usize) -> &T;
    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T];
    fn decode<T: DecodePrimitive>(
        &self,
        offset: &mut usize,
        len: usize,
    ) -> Result<T, crate::goblin_error::GoblinError>;
}

impl ArgsDecoder for ArgsBuffer {
    /// Zero-copy decoding of a reference to a value of type `T` from the buffer.
    fn decode_ref<T>(&self, start: usize) -> &T {
        let end = start + core::mem::size_of::<T>();
        unsafe { &*(self[start..end].as_ptr() as *const T) }
    }

    /// Zero-copy decoding of a slice of values of type `T` from the buffer.
    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T] {
        let byte_len = len * core::mem::size_of::<T>();
        let end = start + byte_len;

        unsafe { core::slice::from_raw_parts(self[start..end].as_ptr() as *const T, len) }
    }

    /// Decode a primitive value from the buffer with bounds checking.
    ///
    /// This method automatically advances the offset by the size of the decoded type.
    ///
    /// # Arguments
    /// * `offset` - Mutable reference to current position, will be advanced
    /// * `len` - Total length of valid data in the buffer
    ///
    /// # Returns
    /// The decoded value or an InvalidPayload error if bounds are exceeded
    fn decode<T: DecodePrimitive>(
        &self,
        offset: &mut usize,
        len: usize,
    ) -> Result<T, crate::goblin_error::GoblinError> {
        let size = core::mem::size_of::<T>();
        crate::require!(
            len >= *offset + size,
            crate::goblin_error::GoblinError::InvalidPayload
        );

        let value = T::from_le_bytes_at(self, *offset);
        *offset += size;
        Ok(value)
    }
}

/// Trait for types that can be decoded from a byte buffer with bounds checking.
///
/// This trait provides a safe way to decode primitive types from byte arrays
/// while automatically handling bounds checking and offset advancement.
pub trait DecodePrimitive: Sized {
    /// Decode a value from the buffer at the given offset using little-endian byte order
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self;
}

impl DecodePrimitive for u8 {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        buffer[offset]
    }
}

impl DecodePrimitive for u32 {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        u32::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ])
    }
}

impl DecodePrimitive for u16 {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        u16::from_le_bytes([buffer[offset], buffer[offset + 1]])
    }
}

impl DecodePrimitive for u64 {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        u64::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
            buffer[offset + 4],
            buffer[offset + 5],
            buffer[offset + 6],
            buffer[offset + 7],
        ])
    }
}
