mod read;
#[cfg(feature = "encode")]
mod write;

use crate::input_processor::HeaderFlags;

/// Shared decoding/writing context for [`HeaderRefs`](super::HeaderRefs).
///
/// Holds the header flags, which gate the optional recipient and drive the
/// custom ERC20 count. The backing slice for the zero-copy refs is taken
/// straight off the codec reader, so it is no longer carried here.
#[derive(Clone, Copy)]
pub struct HeaderRefsCtx {
    pub flags: HeaderFlags,
}
