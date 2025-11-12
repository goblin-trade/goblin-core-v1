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
    /// Zero-copy decoding of a reference to a value of type `T` from the buffer.
    fn decode_ref_unchecked<T>(&self, offset: &mut usize) -> &T;

    /// Zero-copy decoding of a slice of values of type `T` from the buffer.
    fn decode_slice_unchecked<T>(&self, offset: &mut usize, len: usize) -> &[T];

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
    ) -> Result<T, crate::goblin_error::GoblinError>;

    /// Decode a primitive value without bounds check and without advancing the offset
    ///
    /// While decode() checks for size and advances the offset in each call,
    /// decode_unchecked() can be used to batch read multiple fields. Bounds check
    /// and offset update must be performed externally.
    fn decode_unchecked<T: DecodePrimitive>(&self, offset: usize) -> T;
}

impl ArgsDecoder for ArgsBuffer {
    fn decode_ref_unchecked<T>(&self, offset: &mut usize) -> &T {
        let start = *offset;
        *offset += core::mem::size_of::<T>();
        unsafe { &*(self[start..*offset].as_ptr() as *const T) }
    }

    fn decode_slice_unchecked<T>(&self, offset: &mut usize, len: usize) -> &[T] {
        let start = *offset;
        *offset += len * core::mem::size_of::<T>();
        unsafe { core::slice::from_raw_parts(self[start..*offset].as_ptr() as *const T, len) }
    }

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

        let value = Self::decode_unchecked::<T>(self, *offset);
        *offset += size;
        Ok(value)
    }

    fn decode_unchecked<T: DecodePrimitive>(&self, offset: usize) -> T {
        T::from_le_bytes_at(self, offset)
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

impl DecodePrimitive for i64 {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        i64::from_le_bytes([
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
