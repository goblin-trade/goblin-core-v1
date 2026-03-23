use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::update_header::UpdateHeader,
    settlement::local_delta::LocalDelta,
    state::MarketState,
};

pub fn ix_update<M, B, Q, In>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let header = UpdateHeader::try_decode(ctx)?;

    // We have increase and decrease sub-operations
    //
    // In: LegMarker is only relevant for placing a new order.
    // If we cancel or increase existing order.
    // We get an invalid state if order exists on wrong side.

    Ok(())
}
