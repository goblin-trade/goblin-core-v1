use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    instructions::take::take_packet::TakePacket,
    markets::{CommonMarket, MarketVariant},
    matching::match_order,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Ticks},
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    token::TokenMarker,
    types::{Base, LegMarker, Quote, TupleReader},
};

pub fn ix_take<M, B, Q, In>(
    ctx: &HostioContext,
    local_delta: &mut LocalDelta,
    market: &CommonMarket<M, B, Q>,
    market_state: &mut MarketState<M, B, Q>,
    offset: &mut usize,
    len: usize,
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    B: TokenMarker,
    Q: TokenMarker,
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

    match_order::<M, B, Q, In>(
        local_delta,
        &ctx.msg_sender,
        market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
