use crate::{
    goblin_error::GoblinError, input_processor::DecodeCtx, settlement::DeltaV3, types::Address,
};

pub trait MarketCounts {
    /// Process legal combinations of market types
    fn process(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        delta: &mut DeltaV3,
    ) -> Result<(), GoblinError>;
}
