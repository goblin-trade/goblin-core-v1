use crate::codec::{CodecError, CodecResult, GoblinRead, Reader};

use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20ListCtx, token_reader::TokenDataTriple},
    types::Address,
};

use super::HeaderRefsCtx;

impl HeaderRefsCtx {
    /// Zero-copy read of the optional custom recipient.
    pub fn read_recipient<'de>(
        &self,
        reader: &mut Reader<'de>,
    ) -> CodecResult<Option<&'de Address>> {
        if self.flags.read_custom_recipient {
            let bytes = reader.take(20)?;
            // `Address` is `[u8; 20]`, so this is a checked, zero-copy borrow.
            let address: &[u8; 20] = bytes.try_into().map_err(|_| CodecError::UnexpectedEof)?;
            Ok(Some(address))
        } else {
            Ok(None)
        }
    }

    /// Zero-copy read of the token triple. Only the custom ERC20 list is carried
    /// in calldata, so its count drives the decode.
    pub fn read_token_data_triple<'de>(
        &self,
        reader: &mut Reader<'de>,
    ) -> CodecResult<TokenDataTriple<'de>> {
        TokenDataTriple::from_reader_with_ctx(
            reader,
            CustomERC20ListCtx {
                count: self.flags.custom_erc20_count,
            },
        )
    }
}
