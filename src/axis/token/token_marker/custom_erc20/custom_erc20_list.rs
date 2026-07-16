use crate::{
    axis::token::{token_marker::TokenData, CustomERC20},
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    require,
    types::Address,
};

pub const MAX_CUSTOM_ERC20_COUNT: usize = 8;

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}

impl<'a> CustomERC20List<'a> {
    pub fn try_decode_no_advance(
        ctx: &'a DecodeCtx,
        custom_erc20_count: usize,
    ) -> Result<Self, GoblinError> {
        require!(
            custom_erc20_count <= MAX_CUSTOM_ERC20_COUNT,
            GoblinError::CustomERC20CountExceeded
        );

        let custom_erc20_list_len = custom_erc20_count * core::mem::size_of::<Address>();
        require!(
            ctx.len() >= ctx.offset.get() + custom_erc20_list_len,
            GoblinError::InvalidPayload
        );

        let inner = ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(custom_erc20_count);
        Ok(CustomERC20List { inner })
    }
}

impl<'a> IntoIterator for CustomERC20List<'a> {
    type Item = TokenData<CustomERC20>;
    type IntoIter = core::iter::Copied<core::slice::Iter<'a, TokenData<CustomERC20>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().copied()
    }
}
