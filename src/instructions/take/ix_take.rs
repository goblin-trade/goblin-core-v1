use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_header::TakeHeader,
    matching::match_order,
    settlement::local_delta::LocalDelta,
    state::MarketState,
};

pub fn ix_take<M, B, Q, In>(
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
    let header = TakeHeader::<In>::try_decode(ctx)?;

    match_order::<M, B, Q, In>(
        local_delta,
        market_and_key,
        market_state,
        header.num_lots,
        header.min_lots_to_fill,
        header.limit,
    )
}
