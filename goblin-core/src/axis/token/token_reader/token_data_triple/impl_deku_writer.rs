use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use crate::axis::token::token_list::custom_erc20::CustomERC20ListCtx;

use super::TokenDataTriple;

/// Only the custom ERC20 list (`.2`) is carried in calldata, so it is the only
/// component serialized; the ETH and hardcoded lists are contract constants.
/// The ctx is forwarded untouched to the list's writer.
impl<'a> DekuWriter<CustomERC20ListCtx<'a>> for TokenDataTriple<'a> {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: CustomERC20ListCtx<'a>,
    ) -> Result<(), DekuError> {
        self.2.to_writer(writer, ctx)
    }
}
