use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_validator::LegValidator, Base, Leg, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_packet::TakePacket,
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Ticks},
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    types::{Address, StoreReader, Tuple},
};

pub fn ix_take<M, B, Q, In>(
    ctx: &DecodeCtx,
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + LegValidator
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>
        + StoreReader<
            Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>,
            Result = In::LotsPerUnit,
        >,
    In::Opposite: StoreReader<Tuple<Ticks, Ticks, Leg>, Result = Ticks>,
{
    let packet = TakePacket::<In>::try_decode(ctx)?;

    match_order::<M, B, Q, In>(
        msg_sender,
        local_delta,
        market_and_key,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
