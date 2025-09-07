use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::IndexedMarket,
    matching::{match_order, MatchResult},
    settlement::PendingMakerUpdates,
    state::MarketState,
    types::{Address, SideMarker},
};

pub fn ix_take<S: SideMarker>(
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    pending_maker_updates: &mut PendingMakerUpdates,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<MatchResult<S>, GoblinError> {
    let packet = TakePacket::<S>::decode(payload, len, offset)?;

    match_order::<S>(
        pending_maker_updates,
        taker,
        indexed_market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )
}
