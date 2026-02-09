use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_packet::TakePacket,
    market::{CommonMarket, MarketMarker},
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Ticks},
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    token::TokenMarker,
    types::{Address, Base, Leg, LegMatcher, LegValidator, Quote, StoreReader, Tuple},
};

pub fn ix_take<M, B, Q, In>(
    ctx: &DecodeCtx,
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market: &CommonMarket<M, B, Q>,
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
        local_delta,
        msg_sender,
        market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
