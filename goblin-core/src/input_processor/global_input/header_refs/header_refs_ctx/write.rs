use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20ListCtx, token_reader::TokenDataTriple},
    types::Address,
};

use super::HeaderRefsCtx;

impl<'a> HeaderRefsCtx<'a> {
    /// Write the optional custom recipient slot when the flags ask for it.
    pub fn write_recipient<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        recipient: &Option<&Address>,
    ) -> Result<(), DekuError> {
        if !self.flags.read_custom_recipient {
            return Ok(());
        }

        match recipient {
            // A well-formed value pairs the flag with `Some`; if it is missing,
            // still emit the slot as zeroes so the layout stays decodable.
            Some(address) => address.to_writer(writer, ()),
            None => [0u8; 20].to_writer(writer, ()),
        }
    }

    /// Write the token triple. Only the custom ERC20 list is carried in
    /// calldata, so only its count and backing slice reach the encode path.
    pub fn write_token_data_triple<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        triple: &TokenDataTriple<'a>,
    ) -> Result<(), DekuError> {
        triple.to_writer(
            writer,
            CustomERC20ListCtx {
                count: triple.2.inner.len(),
                source: self.source,
            },
        )
    }
}
