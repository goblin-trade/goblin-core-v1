use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::IndexedMarket,
    matching::{match_order, MatchResult},
    settlement::MarketMakerDeltas,
    state::MarketState,
    types::{Address, LegMarker},
};

pub fn ix_take<In: LegMarker>(
    pending_maker_updates: &mut MarketMakerDeltas,
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<MatchResult<In>, GoblinError> {
    let packet = TakePacket::<In>::decode(payload, len, offset)?;

    match_order::<In>(
        pending_maker_updates,
        taker,
        indexed_market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
