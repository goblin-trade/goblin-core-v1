//! Byte-level codec: decode from, and encode into, a caller-owned buffer.
//!
//! [`FixedCodec`] is the encode/decode-expanded counterpart of
//! [`FixedDecode`](super::FixedDecode). It owns the *buffer* concerns only:
//! moving whole bytes between the wire and a value. Sub-byte layout is handled
//! by [`BitPack`](super::BitPack), which the `#[fixed_codec]` attribute macro
//! uses when a field declares an explicit `#[codec(bits = N)]` width narrower
//! than its natural size.
//!
//! Unlike `FixedDecode`, this trait carries no lifetime: it targets fixed-size,
//! owned values (the zero-copy, borrowed decoders stay on `FixedDecode` /
//! `VariableDecode`).

mod impl_fixed_codec;
#[cfg(test)]
mod tests;

use crate::goblin_error::GoblinError;
use crate::input_processor::{ArgsReader, ArgsWriter};
use crate::require;

/// Read `byte_len` little-endian bytes into a `u64` lane, advancing `reader`.
///
/// Used by derived code for a struct with a top-level `#[codec(bits = N)]`,
/// where the whole struct is one bit stream whose field widths are only known
/// at compile time via `BitPack::CAPACITY`.
#[inline]
pub fn read_lane(reader: &ArgsReader, byte_len: usize) -> u64 {
    let offset = reader.offset.get();
    let mut lane = 0u64;
    let mut i = 0;
    while i < byte_len {
        lane |= (reader.args[offset + i] as u64) << (i * 8);
        i += 1;
    }
    reader.advance_offset(byte_len);
    lane
}

/// Write the low `byte_len` bytes of `lane`, little-endian, advancing `writer`.
#[inline]
pub fn write_lane(writer: &mut ArgsWriter, lane: u64, byte_len: usize) {
    let mut i = 0;
    while i < byte_len {
        writer.write_bytes(&[(lane >> (i * 8)) as u8]);
        i += 1;
    }
}

/// Encode and decode a fixed-size value.
pub trait FixedCodec: Sized {
    /// Encoded size in bytes. Not necessarily `core::mem::size_of::<Self>()` —
    /// this is the *wire* size, which may differ from in-memory layout.
    const ENCODED_SIZE: usize;

    /// Decode assuming `ENCODED_SIZE` bytes are available at the current
    /// offset. Advances `reader`'s offset by `ENCODED_SIZE`.
    fn raw_fixed_decode(reader: &ArgsReader) -> Self;

    /// Encode into `writer` at its current offset, advancing by
    /// `ENCODED_SIZE`.
    fn raw_fixed_encode(&self, writer: &mut ArgsWriter);

    /// Check constraints on an already-decoded value.
    /// Default is a no-op; leaf types override it.
    fn validate(&self) -> Result<(), GoblinError> {
        Ok(())
    }

    /// Bounds-checked decode. Default implementation: check once, then
    /// decode unchecked.
    fn try_fixed_decode(reader: &ArgsReader) -> Result<Self, GoblinError> {
        require!(
            reader.len() >= reader.offset.get() + Self::ENCODED_SIZE,
            GoblinError::InvalidPayload
        );
        let value = Self::raw_fixed_decode(reader);
        value.validate()?;
        Ok(value)
    }

    /// Validated encode. Validation runs before any bytes are written so a
    /// rejected value does not leave a partially-written buffer.
    fn try_fixed_encode(&self, writer: &mut ArgsWriter) -> Result<(), GoblinError> {
        self.validate()?;
        self.raw_fixed_encode(writer);
        Ok(())
    }
}
