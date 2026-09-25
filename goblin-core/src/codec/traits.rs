//! The `GoblinRead`/`GoblinWrite` traits.

use super::{CodecResult, Reader, Writer};

/// Decode `Self` from a byte-oriented [`Reader`].
///
/// The lifetime ties the decoded value to the reader's backing input, so
/// borrowing decoders stay zero-copy. `Ctx` carries out-of-band decoding
/// parameters (a preceding flag, a count, ...) for types that need them.
pub trait GoblinRead<'de, Ctx = ()>: Sized {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, ctx: Ctx) -> CodecResult<Self>;
}

/// Encode `self` into a byte-oriented [`Writer`].
pub trait GoblinWrite<Ctx = ()> {
    fn to_writer(&self, writer: &mut Writer<'_>, ctx: Ctx) -> CodecResult<()>;
}
