use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::axis::token::token_list::custom_erc20::{CustomERC20List, CustomERC20ListCtx};

use super::TokenDataTriple;

/// Only the custom ERC20 list is carried in calldata, so decoding the triple
/// reads it zero-copy through [`CustomERC20ListCtx`] and pairs it with the two
/// contract constants.
impl<'de> GoblinRead<'de, CustomERC20ListCtx> for TokenDataTriple<'de> {
    fn from_reader_with_ctx(
        reader: &mut Reader<'de>,
        ctx: CustomERC20ListCtx,
    ) -> CodecResult<Self> {
        let list = CustomERC20List::from_reader_with_ctx(reader, ctx)?;
        Ok(Self::const_from(list))
    }
}
