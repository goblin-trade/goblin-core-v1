use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::{MarketVariant, PairShape},
    matching::{match_order, MatchResult},
    quantities::Ticks,
    settlement::market::{MakerSideDelta, MarketDelta},
    state::MarketState,
    types::{Address, Base, LegMarker, PairAccessor, Quote},
};

pub fn ix_take<M, P, In>(
    market_delta: &MarketDelta<P>,
    msg_sender: &Address,

    // TODO why not pass CommonMarket?
    // hardcoded market hash is never used.
    // CommonMarket will remove the need to define a general trait for M::Market
    market: &M::Market<P>,

    market_state: &mut MarketState<M, P>,
    payload: &ArgsBuffer,
    offset: &mut usize,
    len: usize,
) -> Result<MatchResult<In>, GoblinError>
where
    M: MarketVariant,
    P: PairShape,
    In: LegMarker
        + PairAccessor<MakerSideDelta<Base>, MakerSideDelta<Quote>, Result = MakerSideDelta<In>>,
    In::Opposite: PairAccessor<Ticks, Ticks, Result = Ticks>,
{
    let packet = TakePacket::<In>::decode(payload, len, offset)?;

    match_order::<M, P, In>(
        market_delta,
        msg_sender,
        market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
