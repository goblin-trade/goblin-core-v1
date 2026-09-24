//! Read/write traits for Goblin wire formats.
//!
//! These are the intended replacement for the `deku`-based decoding in
//! `goblin-core`. The trait is deliberately **not** generic over the backing
//! reader: decoding happens directly over a borrowed `&[u8]`, so a borrowing
//! decoder (a zero-copy token list, say) can name the backing slice and hand
//! back `&'de` references without threading a separate "source" through a
//! context.
//!
//! Sub-byte fields are not modelled here: the wire format is byte-oriented and
//! the existing bit packing is decoded by hand out of whole-byte lanes.
//!
//! NOTE: these traits are defined but not yet wired in. `goblin-core` still
//! decodes with `deku`, so nothing implements `GoblinRead`/`GoblinWrite` yet.
//! Primitive/array impls arrive with the migration.

use core::result::Result as CoreResult;

/// Error produced while decoding or encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    /// The input ended before the field could be fully read.
    UnexpectedEof,
    /// The output buffer was too small to hold the encoded value.
    BufferFull,
    /// A decoded value failed a structural check (deku's `assert`).
    InvalidValue,
}

/// Decoding/encoding result.
pub type CodecResult<T> = CoreResult<T, CodecError>;

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

/// Decode `Self` from a byte-oriented [`Reader`].
///
/// The lifetime ties the decoded value to the reader's backing input, so
/// borrowing decoders stay zero-copy.
pub trait GoblinRead<'de>: Sized {
    fn from_reader(reader: &mut Reader<'de>) -> CodecResult<Self>;
}

/// Encode `self` into a byte-oriented [`Writer`].
pub trait GoblinWrite {
    fn to_writer(&self, writer: &mut Writer<'_>) -> CodecResult<()>;
}
