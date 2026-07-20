use crate::{
    axis::token::token_reader::TokenDataTriple, goblin_error::GoblinError,
    input_processor::DecodeCtx, settlement::Delta, types::Address,
};

pub trait MarketCounts {
    /// Process legal combinations of market types
    fn process<'a>(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>;
}
