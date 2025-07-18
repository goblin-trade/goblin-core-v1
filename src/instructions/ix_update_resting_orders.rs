use crate::{
    goblin_error::GoblinError,
    input_processor::{Args, ArgsBuffer, ArgsDecoder},
    markets::{IndexedMarket, Market},
    quantities::{BaseLots, Ticks},
    require,
    types::Address,
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
    custom_erc20_list: &[Address],
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

    let market = Market::from_index(
        header.market_index as usize,
        custom_market_list,
        custom_erc20_list,
    )?;

    Ok(())
}
