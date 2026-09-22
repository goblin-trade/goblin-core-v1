mod read;
#[cfg(feature = "encode")]
mod write;

use crate::input_processor::HeaderFlags;

/// Shared Deku context for [`HeaderRefs`](super::HeaderRefs), used by both the
/// reader and writer.
///
/// Zero-copy borrowing needs the reader's backing `&[u8]`, but a
/// [`DekuReader`](deku::DekuReader) impl is generic over the reader `R` and
/// cannot recover that slice, so it is threaded through the context alongside
/// the header flags. The zero-copy reads and the flag-gated write live in the
/// sibling `read` and `write` modules, so the derived impls only name them.
#[derive(Clone, Copy)]
pub struct HeaderRefsCtx<'a> {
    pub flags: HeaderFlags,
    pub source: &'a [u8],
}
