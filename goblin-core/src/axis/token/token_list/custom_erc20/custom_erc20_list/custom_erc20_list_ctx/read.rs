use core::mem::size_of;

use crate::codec::{CodecResult, Reader};

use crate::axis::token::{CustomERC20, token_marker::TokenData};

use super::CustomERC20ListCtx;

/// Guard for the zero-copy reinterpretation below: `TokenData<CustomERC20>` is
/// `Address` (`[u8; 20]`, align 1) plus a zero-sized decimals marker, so it must
/// occupy exactly 20 contiguous bytes.
const _: () = assert!(size_of::<TokenData<CustomERC20>>() == 20);

impl CustomERC20ListCtx {
    /// Zero-copy read of `count` custom ERC20 tokens out of the calldata slice.
    pub fn read_tokens<'de>(
        &self,
        reader: &mut Reader<'de>,
    ) -> CodecResult<&'de [TokenData<CustomERC20>]> {
        let len = self.count * size_of::<TokenData<CustomERC20>>();
        let bytes = reader.take(len)?;

        // SAFETY: `TokenData<CustomERC20>` is exactly 20 bytes with align 1 (see
        // the assert above), matching the borrowed `bytes`, and every bit
        // pattern of a byte is valid.
        Ok(unsafe {
            core::slice::from_raw_parts(bytes.as_ptr() as *const TokenData<CustomERC20>, self.count)
        })
    }
}
