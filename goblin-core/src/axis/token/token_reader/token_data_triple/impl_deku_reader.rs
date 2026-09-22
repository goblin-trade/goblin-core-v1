use deku::DekuError;
use deku::DekuReader;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::axis::token::token_list::custom_erc20::{CustomERC20List, CustomERC20ListCtx};

use super::TokenDataTriple;

/// Only the custom ERC20 list is carried in calldata, so decoding the triple
/// reads it zero-copy through [`CustomERC20ListCtx`] and pairs it with the two
/// contract-constant lists.
impl<'a> DekuReader<'a, CustomERC20ListCtx<'a>> for TokenDataTriple<'a> {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: CustomERC20ListCtx<'a>,
    ) -> Result<Self, DekuError> {
        let list = CustomERC20List::from_reader_with_ctx(reader, ctx)?;
        Ok(Self::const_from(list))
    }
}
