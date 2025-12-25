use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
};

/// Decoder for ETH::TokenIndex<M> and ETH::Deposit
/// The value of both is `()`, so we simply return `()`
impl Decodable<()> for () {
    fn decode(_args: &ArgsBuffer, _offset: &mut usize, _len: usize) -> Result<(), GoblinError> {
        Ok(())
    }
}
