use crate::codec::{CodecResult, Writer};

use crate::axis::token::{CustomERC20, token_marker::TokenData};

use super::CustomERC20ListCtx;

impl CustomERC20ListCtx {
    /// Write the list as the concatenated token addresses, mirroring the
    /// zero-copy slice read. Each `TokenData<CustomERC20>` is just its `Address`
    /// (20 bytes); the decimals marker is zero-sized. `count` is not consulted
    /// when encoding.
    pub fn write_tokens(
        &self,
        writer: &mut Writer<'_>,
        tokens: &[TokenData<CustomERC20>],
    ) -> CodecResult<()> {
        for token in tokens {
            writer.write(&token.address)?;
        }
        Ok(())
    }
}
