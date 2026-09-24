use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20ListCtx, token_reader::TokenDataTriple},
    types::Address,
};

use super::HeaderRefsCtx;

impl HeaderRefsCtx {
    /// Write the optional custom recipient slot when the flags ask for it.
    pub fn write_recipient(
        &self,
        writer: &mut Writer<'_>,
        recipient: &Option<&Address>,
    ) -> CodecResult<()> {
        if !self.flags.read_custom_recipient {
            return Ok(());
        }

        match recipient {
            // A well-formed value pairs the flag with `Some`; if it is missing,
            // still emit the slot as zeroes so the layout stays decodable.
            Some(address) => (*address).to_writer(writer, ()),
            None => [0u8; 20].to_writer(writer, ()),
        }
    }

    /// Write the token triple. Only the custom ERC20 list is carried in
    /// calldata, so only its count reaches the encode path.
    pub fn write_token_data_triple<'de>(
        &self,
        writer: &mut Writer<'_>,
        triple: &TokenDataTriple<'de>,
    ) -> CodecResult<()> {
        triple.to_writer(
            writer,
            CustomERC20ListCtx {
                count: triple.2.inner.len(),
            },
        )
    }
}
