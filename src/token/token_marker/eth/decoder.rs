use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

/// Decoder for ETH::TokenIndex<M> and ETH::Deposit
/// The value of both is `()`, so we simply return `()`
impl Decodable for () {
    fn try_decode(_ctx: &DecodeCtx) -> Result<(), GoblinError> {
        Ok(())
    }
}
