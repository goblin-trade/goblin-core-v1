use crate::{
    axis::token::token_reader::custom_erc20::custom_erc20_index::CustomERC20Index,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

impl Decodable for CustomERC20Index {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let index_raw = u8::try_decode(ctx)? as usize;
        Ok(CustomERC20Index(index_raw))
    }
}
