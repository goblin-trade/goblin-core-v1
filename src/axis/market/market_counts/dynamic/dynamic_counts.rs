use crate::{
    axis::token::token_index::{CustomERC20Index, CustomERC20List, TokenData},
    goblin_error::GoblinError,
    input_processor::{DecodablePrimitive, DecodeCtx},
    require,
    types::Address,
};

const BYTE_COUNT: usize = 5;

/// The number of dynamic markets to process and their associated custom token addresses
#[derive(Clone, Copy)]
pub struct DynamicCounts<'a> {
    pub market_counts: [u8; 8],

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: CustomERC20List<'a>,
}

impl<'a> DynamicCounts<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let byte_1 = u8::decode_unchecked_no_advance(ctx);
        let byte_2 = u8::decode_unchecked_no_advance(ctx);
        let byte_3 = u8::decode_unchecked_no_advance(ctx);
        let byte_4 = u8::decode_unchecked_no_advance(ctx);

        let market_counts = [
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
            byte_3 & 0b0000_1111,
            byte_3 >> 4,
        ];

        let custom_erc20_count = byte_4 as usize;

        ctx.advance_offset(BYTE_COUNT);

        let custom_erc20_list_len = custom_erc20_count * core::mem::size_of::<Address>();
        require!(
            ctx.len() >= ctx.offset.get() + custom_erc20_list_len,
            GoblinError::InvalidPayload
        );

        let custom_erc20_list = CustomERC20List {
            inner: ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20Index>>(custom_erc20_count),
        };

        Ok(Self {
            market_counts,
            custom_erc20_list,
        })
    }
}
