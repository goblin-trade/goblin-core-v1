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
