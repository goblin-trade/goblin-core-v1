use crate::{
    quantities::{BaseLots, Ticks},
    types::Address,
};

pub const HANDLE_4_PLACE_MULTIPLE_ORDERS: u8 = 4;

#[repr(C, packed)]
pub struct PlaceMultipleOrdersHeader {
    /// The market base token
    base_token: Address,

    /// The market quote token
    quote_token: Address,

    /// Whether to fail on cross or whether to amend to amend the price
    fail_on_cross: bool,

    /// Whether to skip orders with insufficient funds, or whether to revert the whole TX
    skip_on_insufficient_funds: bool,

    /// If no free slots are available at a given price, amend the price away from centre
    /// by this many ticks
    tick_offset: u8,

    /// Whether to only use funds in TraderTokenState, or to also transfer in ERC20 tokens
    /// if necessary
    use_free_funds: bool,

    /// Number of bids
    bids_count: u8,

    /// Number of asks
    asks_count: u8,
}

#[repr(C, packed)]
pub struct PostOnlyOrder {
    pub price: Ticks,

    pub size: BaseLots,

    pub expiry: u32,

    pub expiry_type: ExpiryType,
}

#[repr(u8)]
pub enum ExpiryType {
    None = 0,
    Timestamp = 1,
    Block = 2,
}

pub fn handle_4_place_multiple_orders() -> Result<(), ()> {
    Ok(())
}
