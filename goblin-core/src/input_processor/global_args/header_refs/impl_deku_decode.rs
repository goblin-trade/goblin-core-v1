use deku::{
    DekuError, DekuReader,
    no_std_io::{Read, Seek},
    reader::Reader,
};

use crate::{
    axis::token::{
        CustomERC20, token_list::custom_erc20::CustomERC20List, token_marker::TokenData,
        token_reader::TokenDataTriple,
    },
    input_processor::{HeaderRefsCtx, zero_copy_from, zero_copy_slice_from},
    types::Address,
};

use super::HeaderRefs;

impl<'a> DekuReader<'a, HeaderRefsCtx<'a>> for HeaderRefs<'a> {
    /// Zero-copy decode the optional custom recipient and the custom ERC20 list
    /// from `reader`, advancing it past both.
    ///
    /// The backing slice is taken from `ctx.source`, so this works for any
    /// reader type `R`.
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: HeaderRefsCtx<'a>,
    ) -> Result<Self, DekuError> {
        let custom_recipient = if ctx.flags.read_custom_recipient {
            // SAFETY: `Address` is `[u8; 20]`, so every bit pattern is valid,
            // and `ctx.source` is the reader's backing slice.
            Some(unsafe { zero_copy_from::<Address, _>(reader, ctx.source) })
        } else {
            None
        };

        // SAFETY: `TokenData<CustomERC20>` is `Address` (`[u8; 20]`) plus a
        // zero-sized decimals marker, so every bit pattern is a valid value,
        // and `ctx.source` is the reader's backing slice.
        let inner = unsafe {
            zero_copy_slice_from::<TokenData<CustomERC20>, _>(
                reader,
                ctx.source,
                ctx.flags.custom_erc20_count,
            )
        };
        let custom_erc20_list = CustomERC20List { inner };
        let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

        Ok(Self {
            custom_recipient,
            token_data_triple,
        })
    }
}

#[cfg(test)]
mod tests {
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;
    use crate::input_processor::{ArgsReaderV2, HeaderFlags};

    #[test]
    fn decodes_recipient_and_tokens_without_copying() {
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

        let expected_recipient: Address = core::array::from_fn(|i| i as u8);
        assert_eq!(refs.custom_recipient, Some(&expected_recipient));

        let tokens = refs.token_data_triple.2.inner;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].address, core::array::from_fn(|i| (i + 20) as u8));
        assert_eq!(tokens[1].address, core::array::from_fn(|i| (i + 40) as u8));

        // 20 + 2 * 20 bytes consumed.
        assert_eq!(reader.bits_read, 60 * 8);
    }

    #[test]
    fn skips_recipient_when_flag_cleared() {
        let data: [u8; 20] = core::array::from_fn(|i| i as u8);
        let flags = HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: false,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 1,
        };

        let mut reader: ArgsReaderV2 = Reader::new(Cursor::new(&data[..]));
        let source: &[u8] = &data;
        let refs =
            HeaderRefs::from_reader_with_ctx(&mut reader, HeaderRefsCtx { flags, source }).unwrap();

        assert!(refs.custom_recipient.is_none());
        assert_eq!(
            refs.token_data_triple.2.inner[0].address,
            core::array::from_fn(|i| i as u8)
        );
    }
}
