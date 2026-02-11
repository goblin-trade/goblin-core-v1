use crate::{
    goblin_error::GoblinError, input_processor::DecodeCtx, settlement::Delta, types::Address,
};

pub trait MarketCounts {
    /// Process legal combinations of market types
    fn process(
        &self,
        ctx: &DecodeCtx,
        msg_sender: &Address,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>;
}
