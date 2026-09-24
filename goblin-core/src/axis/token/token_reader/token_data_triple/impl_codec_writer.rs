use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::axis::token::token_list::custom_erc20::CustomERC20ListCtx;

use super::TokenDataTriple;

/// Only the custom ERC20 list (`.2`) is carried in calldata, so it is the only
/// component serialized; the ETH and hardcoded lists are contract constants.
/// The ctx is forwarded untouched to the list's writer.
impl<'de> GoblinWrite<CustomERC20ListCtx> for TokenDataTriple<'de> {
    fn to_writer(&self, writer: &mut Writer<'_>, ctx: CustomERC20ListCtx) -> CodecResult<()> {
        self.2.to_writer(writer, ctx)
    }
}
