mod custom_erc20_list_ctx;
mod impl_index;
mod impl_into_iterator;

pub use custom_erc20_list_ctx::CustomERC20ListCtx;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::axis::token::{CustomERC20, token_marker::TokenData};

/// Zero copy custom ERC20 token list read from calldata.
///
/// Reading borrows zero-copy out of the calldata slice carried by
/// [`CustomERC20ListCtx`], which is why the field uses a custom
/// `#[deku(reader)]`. The same ctx drives writing; neither of its fields is
/// consulted there.
#[derive(Clone, Copy, DekuRead, Default)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[deku(ctx = "ctx: CustomERC20ListCtx<'a>")]
pub struct CustomERC20List<'a> {
    /// Concatenated token addresses, no length prefix.
    #[deku(reader = "ctx.read_tokens(deku::reader)")]
    #[cfg_attr(
        feature = "encode",
        deku(writer = "ctx.write_tokens(deku::writer, self.inner)")
    )]
    pub inner: &'a [TokenData<CustomERC20>],
}
