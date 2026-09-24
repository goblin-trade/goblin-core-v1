//! Writer over a borrowed mutable byte slice.

use super::{CodecError, CodecResult};

/// A byte-oriented writer over a borrowed mutable slice.
#[derive(Debug)]
pub struct Writer<'a> {
    output: &'a mut [u8],
    pos: usize,
}

impl<'a> Writer<'a> {
    /// Wrap `output`, writing from the first byte.
    pub fn new(output: &'a mut [u8]) -> Self {
        Self { output, pos: 0 }
    }

    /// Offset of the next byte to be written.
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Number of bytes still available.
    pub fn remaining_len(&self) -> usize {
        self.output.len().saturating_sub(self.pos)
    }

    /// Append `bytes`, failing if the buffer is full.
    pub fn write(&mut self, bytes: &[u8]) -> CodecResult<()> {
        let end = self
            .pos
            .checked_add(bytes.len())
            .ok_or(CodecError::BufferFull)?;
        let slot = self
            .output
            .get_mut(self.pos..end)
            .ok_or(CodecError::BufferFull)?;
        slot.copy_from_slice(bytes);
        self.pos = end;
        Ok(())
    }

    /// Write a single byte.
    pub fn write_u8(&mut self, value: u8) -> CodecResult<()> {
        self.write(&[value])
    }

    /// Write a `u16` little-endian.
    pub fn write_u16_le(&mut self, value: u16) -> CodecResult<()> {
        self.write(&value.to_le_bytes())
    }

    /// Write a `u32` little-endian.
    pub fn write_u32_le(&mut self, value: u32) -> CodecResult<()> {
        self.write(&value.to_le_bytes())
    }

    /// Write a `u64` little-endian.
    pub fn write_u64_le(&mut self, value: u64) -> CodecResult<()> {
        self.write(&value.to_le_bytes())
    }
}
