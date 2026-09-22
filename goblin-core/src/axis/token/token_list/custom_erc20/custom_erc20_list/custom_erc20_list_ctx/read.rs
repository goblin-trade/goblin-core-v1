use deku::DekuError;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::{
    axis::token::{CustomERC20, token_marker::TokenData},
    input_processor::ZeroCopyReader,
};

use super::CustomERC20ListCtx;

impl<'a> CustomERC20ListCtx<'a> {
    /// Zero-copy read of `count` custom ERC20 tokens out of the calldata slice.
    pub fn read_tokens<R: Read + Seek>(
        &self,
        reader: &mut Reader<R>,
    ) -> Result<&'a [TokenData<CustomERC20>], DekuError> {
        let mut reader = ZeroCopyReader::new(reader, self.source);
        // SAFETY: `TokenData<CustomERC20>` is `Address` (`[u8; 20]`) plus a
        // zero-sized decimals marker, so every bit pattern is a valid value,
        // and `self.source` is the reader's backing slice.
        Ok(unsafe { reader.zero_copy_slice(self.count) })
    }
}
