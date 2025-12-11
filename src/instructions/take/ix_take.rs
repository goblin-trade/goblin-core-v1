use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    instructions::take::take_packet::TakePacket,
    markets::{CommonMarket, MarketVariant, PairShape},
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Ticks},
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    types::{Base, LegMarker, Quote, TupleReader},
};

pub fn ix_take<M, P, In>(
    ctx: &HostioContext,
    local_delta: &mut LocalDelta,
    market: &CommonMarket<M, P>,
    market_state: &mut MarketState<M, P>,
    offset: &mut usize,
    len: usize,
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    P: PairShape,
    In: LegMarker
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
    let packet = TakePacket::<In>::decode(&ctx.args, len, offset)?;

    match_order::<M, P, In>(
        local_delta,
        &ctx.msg_sender,
        market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
