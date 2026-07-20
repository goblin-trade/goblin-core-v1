use crate::{
    axis::token::{token_marker::TokenData, CustomERC20},
    goblin_error::GoblinError,
    input_processor::{DecodablePrimitive, DecodeCtx},
    require,
    types::Address,
};

pub const MAX_CUSTOM_ERC20_COUNT: usize = 8;

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}

impl<'a> CustomERC20List<'a> {
    pub fn try_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let byte = u8::decode_unchecked_no_advance(ctx);
        require!(
            ctx.len() >= ctx.offset.get() + core::mem::size_of::<u8>(),
            GoblinError::InvalidPayload
        );

        let custom_erc20_count = byte as usize;

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

    pub fn decode_empty(ctx: &'a DecodeCtx) -> Self {
        let inner = ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(0);
        CustomERC20List { inner }
    }
}
