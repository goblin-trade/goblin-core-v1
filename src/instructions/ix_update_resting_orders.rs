use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::IndexedMarket,
    quantities::{BaseLots, Delta, Ticks},
    require,
    settlement::TokenDeltas,
};

#[repr(C, packed)]
pub struct UpdateRestingOrdersHeader {
    pub market_index: u8,

    /// The number of resting bids to update.
    pub bids: u8,
    /// The number of resting asks to update.
    pub asks: u8,
}

impl UpdateRestingOrdersHeader {
    fn order_count(&self) -> u8 {
        self.bids + self.asks
    }
}

const UPDATE_RESTING_ORDERS_HEADER_SIZE: usize = core::mem::size_of::<UpdateRestingOrdersHeader>();

#[repr(C, packed)]
pub struct UpdateArgs {
    pub price: Ticks,
    pub row_index: u8,
    pub size: BaseLots,
}

/// Need to think more on how retail actually places limit orders. They will not mention row index.
/// We need to fit order at the best available row index.
///
/// Whereas for market makers, they have a use case to increase or decrease existing orders.
///
/// * This function: open position, increase and decrease size in bulk
/// * post_limit_orders(): only price and slippage tolerance will be mentioned. Existing order at the
/// same price will get overlooked in order to fit at the next best available row index.
///
/// For now, just find a way to update deltas for market tokens. We have the market index.
pub fn update_resting_order(
    payload: &ArgsBuffer,
    len: usize,
    offset: &mut usize,
    custom_market_list: &[IndexedMarket],
    token_deltas: &mut TokenDeltas,
) -> Result<(), GoblinError> {
    // First decode header
    require!(
        len >= *offset + UPDATE_RESTING_ORDERS_HEADER_SIZE,
        GoblinError::InvalidPayload
    );
    let header = payload.decode_ref::<UpdateRestingOrdersHeader>(*offset);
    *offset += UPDATE_RESTING_ORDERS_HEADER_SIZE;

    // Then decode update args one by one. Bids come first, then asks.
    let order_byte_size = header.order_count() as usize * core::mem::size_of::<UpdateArgs>();
    require!(
        len >= *offset + order_byte_size,
        GoblinError::InvalidPayload
    );
    let bids = payload.decode_slice::<UpdateArgs>(*offset, header.bids as usize);
    let asks =
        payload.decode_slice::<UpdateArgs>(*offset + header.bids as usize, header.asks as usize);
    *offset += order_byte_size;

    let indexed_market =
        IndexedMarket::from_index(header.market_index as usize, custom_market_list)?;

    // Mock amounts that need to be settled
    let base_delta = Delta(10);
    let quote_delta = Delta(-4);

    token_deltas.add_consumed_amount(indexed_market.base_token_index, base_delta)?;
    token_deltas.add_consumed_amount(indexed_market.quote_token_index, quote_delta)?;

    Ok(())
}
