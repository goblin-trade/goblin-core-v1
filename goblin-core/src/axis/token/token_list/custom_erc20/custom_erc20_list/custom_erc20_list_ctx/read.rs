use deku::DekuError;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::{
    axis::token::{CustomERC20, token_marker::TokenData},
    input_processor::zero_copy_slice_from,
};

use super::CustomERC20ListCtx;

impl<'a> CustomERC20ListCtx<'a> {
    /// Zero-copy read of `count` custom ERC20 tokens out of the calldata slice.
    pub fn read_tokens<R: Read + Seek>(
        &self,
        reader: &mut Reader<R>,
    ) -> Result<&'a [TokenData<CustomERC20>], DekuError> {
        // SAFETY: `TokenData<CustomERC20>` is `Address` (`[u8; 20]`) plus a
        // zero-sized decimals marker, so every bit pattern is a valid value,
        // and `self.source` is the reader's backing slice.
        Ok(unsafe {
            zero_copy_slice_from::<TokenData<CustomERC20>, _>(reader, self.source, self.count)
        })
    }
}
