use deku::DekuError;
use deku::DekuReader;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::{
    axis::token::{
        token_list::custom_erc20::{CustomERC20List, CustomERC20ListCtx},
        token_reader::TokenDataTriple,
    },
    input_processor::zero_copy_from,
    types::Address,
};

use super::HeaderRefsCtx;

impl<'a> HeaderRefsCtx<'a> {
    /// Zero-copy read of the optional custom recipient.
    pub fn read_recipient<R: Read + Seek>(
        &self,
        reader: &mut Reader<R>,
    ) -> Result<Option<&'a Address>, DekuError> {
        if self.flags.read_custom_recipient {
            // SAFETY: `Address` is `[u8; 20]`, so every bit pattern is valid,
            // and `self.source` is the reader's backing slice.
            Ok(Some(unsafe {
                zero_copy_from::<Address, _>(reader, self.source)
            }))
        } else {
            Ok(None)
        }
    }

    /// Zero-copy read of the custom ERC20 list, wrapped with the two
    /// contract-constant lists into a [`TokenDataTriple`].
    pub fn read_token_data_triple<R: Read + Seek>(
        &self,
        reader: &mut Reader<R>,
    ) -> Result<TokenDataTriple<'a>, DekuError> {
        // The custom list borrows from the same calldata slice and count that
        // drive this ctx, so hand them to its own zero-copy reader.
        let list = CustomERC20List::from_reader_with_ctx(
            reader,
            CustomERC20ListCtx {
                count: self.flags.custom_erc20_count,
                source: self.source,
            },
        )?;
        Ok(TokenDataTriple::const_from(list))
    }
}
