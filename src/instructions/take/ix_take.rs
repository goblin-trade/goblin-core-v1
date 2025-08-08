use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::take::take_packet::TakePacket,
    markets::IndexedMarket,
    settlement::TokenDeltas,
    state::{MarketKey, MarketState, SlotState},
    tokens::ValidatedTokenPair,
    types::Address,
};

pub fn ix_take(
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
    custom_market_list: &[IndexedMarket],
    custom_erc20_list: &[Address],
    token_deltas: &mut TokenDeltas,
) -> Result<(), GoblinError> {
    let packet = TakePacket::decode(payload, len, offset)?;
    let indexed_market = packet.market_index.to_indexed_market(custom_market_list)?;

    let token_pair = ValidatedTokenPair::new(
        indexed_market.base_token_index,
        indexed_market.quote_token_index,
        custom_erc20_list,
    )?;

    // Obtain market key
    let market_key = MarketKey::new(
        &token_pair,
        indexed_market.base_lot_size,
        indexed_market.quote_lot_size,
        indexed_market.tick_size,
    );

    // Read and store market state
    let mut market_state = MarketState::load(&market_key);
    market_state.as_mut().store(&market_key);

    Ok(())
}
