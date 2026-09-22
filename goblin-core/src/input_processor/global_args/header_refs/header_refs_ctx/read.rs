use deku::DekuError;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::{
    axis::token::{
        CustomERC20, token_list::custom_erc20::CustomERC20List, token_marker::TokenData,
        token_reader::TokenDataTriple,
    },
    input_processor::{zero_copy_from, zero_copy_slice_from},
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
        // SAFETY: `TokenData<CustomERC20>` is `Address` (`[u8; 20]`) plus a
        // zero-sized decimals marker, so every bit pattern is a valid value,
        // and `self.source` is the reader's backing slice.
        let inner = unsafe {
            zero_copy_slice_from::<TokenData<CustomERC20>, _>(
                reader,
                self.source,
                self.flags.custom_erc20_count,
            )
        };
        Ok(TokenDataTriple::const_from(CustomERC20List { inner }))
    }
}
