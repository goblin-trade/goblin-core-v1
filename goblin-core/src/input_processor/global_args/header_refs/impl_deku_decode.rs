use deku::DekuError;

use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    input_processor::{ArgsReaderV2, HeaderFlags, ZeroCopyReadV2},
    types::Address,
};

use super::HeaderRefs;

impl<'a> HeaderRefs<'a> {
    /// Zero-copy decode the optional custom recipient and the custom ERC20 list
    /// from `reader`, advancing it past both.
    pub fn decode(reader: &mut ArgsReaderV2<'a>, flags: &HeaderFlags) -> Result<Self, DekuError> {
        let custom_recipient = if flags.read_custom_recipient {
            // SAFETY: `Address` is `[u8; 20]`, so every bit pattern is valid.
            Some(unsafe { reader.zero_copy::<Address>() })
        } else {
            None
        };

        let custom_erc20_list = CustomERC20List::decode_v2(reader, flags.custom_erc20_count)?;
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
        let refs = HeaderRefs::decode(&mut reader, &flags).unwrap();

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
        let refs = HeaderRefs::decode(&mut reader, &flags).unwrap();

        assert!(refs.custom_recipient.is_none());
        assert_eq!(
            refs.token_data_triple.2.inner[0].address,
            core::array::from_fn(|i| i as u8)
        );
    }
}
