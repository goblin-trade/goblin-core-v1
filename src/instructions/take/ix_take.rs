use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_packet::TakePacket,
    market::{CommonMarket, MarketVariant},
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Ticks},
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    token::TokenMarker,
    types::{Address, Base, LegMatcher, LegValidator, Quote, TupleReader},
};

pub fn ix_take<M, B, Q, In>(
    ctx: &DecodeCtx,
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market: &CommonMarket<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
) -> Result<(), GoblinError>
where
    M: MarketVariant<B, Q>,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + LegValidator
        + TupleReader<MakerDelta<Base>, MakerDelta<Quote>, (Base, Quote), Result = MakerDelta<In>>
        + TupleReader<TakerDelta<Base>, TakerDelta<Quote>, (Base, Quote), Result = TakerDelta<In>>
        + TupleReader<
            BaseLotsPerBaseUnit,
            QuoteLotsPerQuoteUnit,
            (Base, Quote),
            Result = In::LotsPerUnit,
        >,
    In::Opposite: TupleReader<Ticks, Ticks, (Base, Quote), Result = Ticks>,
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
