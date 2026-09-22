#[cfg(feature = "encode")]
use deku::DekuWrite;
use deku::{DekuRead, DekuReader};

use crate::{goblin_error::GoblinError, input_processor::ArgsReader};

use super::{Header, HeaderFlags, HeaderRefs, HeaderRefsCtx};

/// Arguments read from calldata.
///
/// The layout depends on [`HeaderFlags`], and the zero-copy [`HeaderRefs`] need
/// the reader's backing slice; both are threaded in through the Deku context.
/// Fields that are not part of the calldata (hostio) live in
/// [`GlobalInput`](super::GlobalInput) instead, so every field here implements
/// Deku.
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[deku(ctx = "source: &'a [u8]")]
pub struct GlobalArgs<'a> {
    pub flags: HeaderFlags,

    /// Layout depends on [`HeaderFlags`], which was decoded just above.
    #[deku(ctx = "flags")]
    pub header: Header,

    /// Zero-copy refs borrow the `source` slice carried by the ctx.
    #[deku(ctx = "HeaderRefsCtx { flags: *flags, source }")]
    pub refs: HeaderRefs<'a>,
}

impl<'a> GlobalArgs<'a> {
    pub fn new(reader: &mut ArgsReader<'a>) -> Result<Self, GoblinError> {
        // The zero-copy `refs` borrow the reader's backing slice, so recover it
        // here rather than making callers pass it in separately.
        let source: &'a [u8] = reader.as_mut().get_ref();

        Self::from_reader_with_ctx(reader, source).map_err(|_| GoblinError::InvalidPayload)
    }
}

#[cfg(test)]
mod tests {
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;

    #[test]
    fn derived_decode_threads_ctx_between_fields() {
        // HeaderFlags = read_msg_value, followed by two zero bytes of
        // MarketCounts (dynamic markets off, no custom recipient, no custom
        // ERC20 list), so the whole payload is three bytes.
        let bytes = [0b0000_0010u8, 0x00, 0x00];
        let mut reader = Reader::new(Cursor::new(&bytes[..]));

        let args = GlobalArgs::from_reader_with_ctx(&mut reader, &bytes[..]).unwrap();

        assert!(args.flags.read_msg_value);
        assert!(args.refs.custom_recipient.is_none());
    }
}
