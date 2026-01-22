use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    token::HardcodedERC20Index,
};

impl<'a> Decodable<'a> for HardcodedERC20Index {
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        let index_raw = u8::try_decode(ctx)? as usize;
        Ok(HardcodedERC20Index(index_raw))
    }
}
