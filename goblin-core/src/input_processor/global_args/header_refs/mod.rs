mod impl_deku_decode;

#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{axis::token::TokenDataTriple, input_processor::HeaderFlags, types::Address};

/// Zero copy values read from calldata, decoded after the header
///
/// # Encoding
///
/// The on-wire layout only holds the optional recipient and the *custom* ERC20
/// list; the ETH and hardcoded ERC20 lists bundled inside `token_data_triple`
/// are contract constants and are not serialized. The derive therefore:
/// - gates the recipient on `flags.read_custom_recipient`, and
/// - writes only `token_data_triple.2` via a custom `writer`.
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[cfg_attr(feature = "encode", deku(ctx = "flags: &HeaderFlags"))]
pub struct HeaderRefs<'a> {
    /// Optional custom recipient
    #[cfg_attr(feature = "encode", deku(cond = "flags.read_custom_recipient"))]
    pub custom_recipient: Option<&'a Address>,

    #[cfg_attr(
        feature = "encode",
        deku(writer = "self.token_data_triple.2.to_writer(deku::writer, ())")
    )]
    pub token_data_triple: TokenDataTriple<'a>,
}

/// Deku decoding context for [`HeaderRefs`].
///
/// Zero-copy borrowing needs the reader's backing `&[u8]`, but a
/// [`DekuReader`] impl is generic over the reader `R` and cannot recover that
/// slice. It is therefore threaded through the context, alongside the header
/// flags.
///
/// [`DekuReader`]: deku::DekuReader
pub struct HeaderRefsCtx<'a> {
    pub flags: HeaderFlags,
    pub source: &'a [u8],
}

#[cfg(all(test, feature = "encode"))]
mod tests {
    use deku::DekuReader;
    use deku::DekuWriter;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;
    use deku::writer::Writer;

    use super::*;
    use crate::input_processor::{ArgsReaderV2, HeaderRefsCtx};

    #[test]
    fn round_trips_through_decode_and_encode() {
        // [recipient: 20][token0: 20][token1: 20]
        let data: [u8; 60] = core::array::from_fn(|i| i as u8);
        let flags = HeaderFlags {
            read_custom_recipient: true,
            read_msg_value: false,
            process_dynamic_markets: false,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 2,
        };

        let mut reader: ArgsReaderV2 = Reader::new(Cursor::new(&data[..]));
        let source: &[u8] = &data;
        let refs =
            HeaderRefs::from_reader_with_ctx(&mut reader, HeaderRefsCtx { flags, source }).unwrap();

        let mut out = [0u8; 60];
        let mut writer = Writer::new(Cursor::new(&mut out[..]));
        refs.to_writer(&mut writer, &flags).unwrap();
        writer.finalize().unwrap();

        assert_eq!(out, data);
    }
}
