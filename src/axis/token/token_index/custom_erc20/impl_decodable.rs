use crate::{
    axis::token::token_index::{CustomERC20Index, MAX_CUSTOM_ERC20_COUNT},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    require,
};

impl Decodable for CustomERC20Index {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let index_raw = u8::try_decode(ctx)? as usize;
        require!(
            index_raw <= MAX_CUSTOM_ERC20_COUNT,
            GoblinError::CustomERC20CountExceeded
        );

        Ok(CustomERC20Index(index_raw))
    }
}
