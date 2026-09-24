#[cfg(feature = "encode")]
use crate::codec::GoblinWrite;
#[cfg(feature = "encode")]
use crate::codec::Writer;
use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::{goblin_error::GoblinError, input_processor::ArgsReader};

use super::{Header, HeaderFlags, HeaderRefs, HeaderRefsCtx};

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

impl<'de> GoblinRead<'de, ()> for GlobalArgs<'de> {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let flags = HeaderFlags::from_reader_with_ctx(reader, ())?;
        let header = Header::from_reader_with_ctx(reader, flags)?;
        let refs = HeaderRefs::from_reader_with_ctx(reader, HeaderRefsCtx { flags })?;

        Ok(Self {
            flags,
            header,
            refs,
        })
    }
}

#[cfg(feature = "encode")]
impl<'de> GoblinWrite<()> for GlobalArgs<'de> {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.flags.to_writer(writer, ())?;
        self.header.to_writer(writer, self.flags)?;
        self.refs
            .to_writer(writer, HeaderRefsCtx { flags: self.flags })?;
        Ok(())
    }
}

impl<'de> GlobalArgs<'de> {
    pub fn new(reader: &mut ArgsReader<'de>) -> Result<Self, GoblinError> {
        Self::from_reader_with_ctx(reader, ()).map_err(|_| GoblinError::InvalidPayload)
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;

    use super::*;

    #[test]
    fn decode_threads_flags_between_fields() {
        // HeaderFlags = read_msg_value, followed by two zero bytes of
        // MarketCounts (dynamic markets off, no custom recipient, no custom
        // ERC20 list), so the whole payload is three bytes.
        let bytes = [0b0000_0010u8, 0x00, 0x00];
        let mut reader = Reader::new(&bytes[..]);

        let args = GlobalArgs::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert!(args.flags.read_msg_value);
        assert!(args.refs.custom_recipient.is_none());
    }
}
