//! Zero-copy reader over a borrowed byte slice.

use super::{CodecError, CodecResult};

/// A byte-oriented, zero-copy reader over a borrowed slice.
#[derive(Debug, Clone)]
pub struct Reader<'de> {
    input: &'de [u8],
    pos: usize,
}

impl<'de> Reader<'de> {
    /// Wrap `input`, starting at the first byte.
    pub const fn new(input: &'de [u8]) -> Self {
        Self { input, pos: 0 }
    }

    /// Offset of the next unread byte.
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Number of bytes not yet read.
    pub fn remaining_len(&self) -> usize {
        self.input.len().saturating_sub(self.pos)
    }

    /// Whether every byte has been consumed.
    pub fn is_exhausted(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// The still-unread input, borrowed from the original slice.
    pub fn remaining(&self) -> &'de [u8] {
        &self.input[self.pos.min(self.input.len())..]
    }

    /// Borrow the next `n` bytes without copying, advancing the position.
    ///
    /// This is the zero-copy primitive: the returned slice has the same
    /// lifetime as the reader's backing input.
    pub fn take(&mut self, n: usize) -> CodecResult<&'de [u8]> {
        let end = self.pos.checked_add(n).ok_or(CodecError::UnexpectedEof)?;
        let out = self
            .input
            .get(self.pos..end)
            .ok_or(CodecError::UnexpectedEof)?;
        self.pos = end;
        Ok(out)
    }

    /// Read a single byte.
    pub fn read_u8(&mut self) -> CodecResult<u8> {
        let byte = *self.input.get(self.pos).ok_or(CodecError::UnexpectedEof)?;
        self.pos += 1;
        Ok(byte)
    }

    /// Read a little-endian `u16`.
    pub fn read_u16_le(&mut self) -> CodecResult<u16> {
        let bytes: [u8; 2] = self
            .take(2)?
            .try_into()
            .map_err(|_| CodecError::UnexpectedEof)?;
        Ok(u16::from_le_bytes(bytes))
    }

    /// Read a little-endian `u32`.
    pub fn read_u32_le(&mut self) -> CodecResult<u32> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| CodecError::UnexpectedEof)?;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read a little-endian `u64`.
    pub fn read_u64_le(&mut self) -> CodecResult<u64> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| CodecError::UnexpectedEof)?;
        Ok(u64::from_le_bytes(bytes))
    }
}
