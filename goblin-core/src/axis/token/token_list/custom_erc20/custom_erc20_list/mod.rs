#[cfg(feature = "encode")]
mod impl_deku_writer;
mod impl_index;
mod impl_into_iterator;

use deku::DekuError;

use crate::{
    axis::token::{CustomERC20, token_marker::TokenData},
    input_processor::{ArgsReaderV2, ZeroCopyReadV2},
};

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}

impl<'a> CustomERC20List<'a> {
    /// Zero-copy decode `count` tokens from `reader`, advancing it by
    /// `count * size_of::<TokenData<CustomERC20>>()` bytes.
    pub fn decode_v2(reader: &mut ArgsReaderV2<'a>, count: usize) -> Result<Self, DekuError> {
        // SAFETY: `TokenData<CustomERC20>` is `Address` (`[u8; 20]`) plus a
        // zero-sized decimals marker, so every bit pattern is a valid value.
        let inner = unsafe { reader.zero_copy_slice::<TokenData<CustomERC20>>(count) };
        Ok(Self { inner })
    }
}
