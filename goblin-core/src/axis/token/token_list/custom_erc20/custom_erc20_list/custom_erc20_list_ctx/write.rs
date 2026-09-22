use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use crate::axis::token::{CustomERC20, token_marker::TokenData};

use super::CustomERC20ListCtx;

impl<'a> CustomERC20ListCtx<'a> {
    /// Write the list as the concatenated token addresses, mirroring the
    /// zero-copy slice read. Each `TokenData<CustomERC20>` is just its `Address`
    /// (20 bytes); the decimals marker is zero-sized. Neither `count` nor
    /// `source` is consulted when encoding.
    pub fn write_tokens<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        tokens: &[TokenData<CustomERC20>],
    ) -> Result<(), DekuError> {
        for token in tokens {
            token.address.to_writer(writer, ())?;
        }
        Ok(())
    }
}
