mod impl_goblin_read;

#[cfg(feature = "encode")]
mod impl_goblin_write;

use super::{Header, HeaderFlags, HeaderRefs};
use crate::codec::GoblinRead;
use crate::{goblin_error::GoblinError, input_processor::ArgsReader};

/// Arguments read from calldata.
///
/// The layout depends on [`HeaderFlags`]: the header is decoded with the flags
/// as context, and the zero-copy [`HeaderRefs`] borrow out of the reader's
/// backing slice. Fields that are not part of the calldata (hostio) live in
/// [`GlobalInput`](super::GlobalInput) instead, so every field here is codec.
pub struct GlobalArgs<'a> {
    pub flags: HeaderFlags,

    /// Layout depends on [`HeaderFlags`], which was decoded just above.
    pub header: Header,

    /// Zero-copy refs borrow the reader's backing slice.
    pub refs: HeaderRefs<'a>,
}

impl<'de> GlobalArgs<'de> {
    pub fn new(reader: &mut ArgsReader<'de>) -> Result<Self, GoblinError> {
        Self::from_reader_with_ctx(reader, ()).map_err(|_| GoblinError::InvalidPayload)
    }
}
