use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

/// Decoder for ETH::TokenIndex<M> and ETH::Deposit
/// The value of both is `()`, so we simply return `()`
impl<'a> Decodable<'a> for () {
    fn decode(_ctx: &'a DecodeCtx<'a>) -> Result<(), GoblinError> {
        Ok(())
    }
}
