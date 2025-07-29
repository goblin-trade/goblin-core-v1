use crate::{
    goblin_error::GoblinError,
    input_processor::ArgsBuffer,
    instructions::reduce_orders::{
        headers::{MatrixHeader, ReduceOrdersHeader},
        reduce_order_packet::ReduceOrderPacket,
    },
    markets::IndexedMarket,
    quantities::Delta,
    settlement::TokenDeltas,
    state::{MarketKey, MarketState, SlotState},
    tokens::ValidatedTokenPair,
    types::Address,
};

/// Reduce resting orders in a market. Set amount to MAX will cancel the order.
pub fn ix_reduce_orders(
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
    custom_market_list: &[IndexedMarket],
    custom_erc20_list: &[Address],
    token_deltas: &mut TokenDeltas,
) -> Result<(), GoblinError> {
    let header = ReduceOrdersHeader::decode(payload, len, offset)?;
    let indexed_market = header.market_index.to_indexed_market(custom_market_list)?;

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

    for _ in 0..header.bid_outer_indices {
        let matrix_header = MatrixHeader::decode(payload, len, offset)?;

        for _ in 0..matrix_header.order_count {
            let ReduceOrderPacket {
                row_index: inner_index,
                column_index: row_index,
                size,
            } = ReduceOrderPacket::decode(payload, len, offset)?;

            // Feed to remover
        }
    }

    // Mock amounts that need to be settled
    let base_delta = Delta(10);
    let quote_delta = Delta(-4);

    // TokenIndex to Token address lookup happens again in settlement phase.
    token_deltas.add_consumed_amount(indexed_market.base_token_index, base_delta)?;
    token_deltas.add_consumed_amount(indexed_market.quote_token_index, quote_delta)?;

    Ok(())
}
