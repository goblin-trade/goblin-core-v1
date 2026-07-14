use crate::{
    axis::token::token_index::{HardcodedERC20Index, HARDCODED_TOKEN_COUNT},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    require,
};

impl Decodable for HardcodedERC20Index {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let index_raw = u8::try_decode(ctx)? as usize;
        require!(
            index_raw <= HARDCODED_TOKEN_COUNT,
            GoblinError::InvalidHardcodedTokenIndex
        );

        Ok(HardcodedERC20Index(index_raw))
    }
}
