//! Error type and result alias for decoding and encoding.

use core::result::Result as CoreResult;

/// Error produced while decoding or encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    /// The input ended before the field could be fully read.
    UnexpectedEof,
    /// The output buffer was too small to hold the encoded value.
    BufferFull,
    /// A decoded value failed a structural check (e.g. a bounds assertion on a
    /// decoded index).
    InvalidValue,
}

/// Decoding/encoding result.
pub type CodecResult<T> = CoreResult<T, CodecError>;
