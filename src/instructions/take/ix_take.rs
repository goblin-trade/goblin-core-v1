use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::{CommonMarket, MarketVariant, PairShape},
    matching::match_order,
    quantities::Ticks,
    settlement::local_delta::{LocalDelta, MakerDelta, TakerDelta},
    state::MarketState,
    types::{Address, Base, LegMarker, PairAccessor, Quote},
};

pub fn ix_take<M, P, In>(
    local_delta: &mut LocalDelta<P>,
    msg_sender: &Address,
    market: &CommonMarket<M, P>,
    market_state: &mut MarketState<M, P>,
    payload: &ArgsBuffer,
    offset: &mut usize,
    len: usize,
) -> Result<(), GoblinError>
where
    M: MarketVariant,
    P: PairShape,
    In: LegMarker
        + PairAccessor<MakerDelta<Base>, MakerDelta<Quote>, Result = MakerDelta<In>>
        + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
    In::Opposite: PairAccessor<Ticks, Ticks, Result = Ticks>,
{
    let packet = TakePacket::<In>::decode(payload, len, offset)?;

    match_order::<M, P, In>(
        local_delta,
        msg_sender,
        market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
