use crate::{
    axis::{
        market::{market_counts::MarketCountsTuple, market_marker::MarketMarker},
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    settlement::Delta,
    types::Address,
};

pub trait MarketCounts: MarketMarker {
    /// Process legal combinations of market types
    fn process<'a>(
        market_counts: &MarketCountsTuple,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>;
}
