use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::IndexedMarket,
    matching::match_order,
    quantities::MarketLotsDelta,
    settlement::MarketMakerDeltas,
    state::MarketState,
    tokens::ValidatedTokenPair,
    types::{Address, SideMarker},
};

pub fn ix_take<S: SideMarker>(
    taker: &Address,
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    taker_delta: &mut MarketLotsDelta,
    maker_deltas: &mut MarketMakerDeltas,
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
) -> Result<(), GoblinError> {
    let packet = TakePacket::<S>::decode(payload, len, offset)?;

    let match_result = match_order::<S>(
        maker_deltas,
        taker,
        indexed_market,
        market_state,
        packet.num_lots,
        packet.min_lots_to_fill,
        packet.price_limit,
    )?;

    taker_delta.apply_match_result(&match_result)
}
