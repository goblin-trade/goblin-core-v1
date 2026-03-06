use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_packet::TakePacket,
    matching::{bitmap::StoredCoordinates, match_order},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit},
    settlement::{
        local_delta::{LocalDelta, MakerDelta},
        MatchedLots,
    },
    state::MarketState,
    types::{StoreReader, Tuple},
};

pub fn ix_take<M, B, Q, In>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>, Result = MatchedLots<In>>
        + StoreReader<
            Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>,
            Result = In::LotsPerUnit,
        >,
    In: StoreReader<Tuple<StoredCoordinates, StoredCoordinates, Leg>, Result = StoredCoordinates>,
{
    let packet = TakePacket::<In>::try_decode(ctx)?;

    match_order::<M, B, Q, In>(
        local_delta,
        market_and_key,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
