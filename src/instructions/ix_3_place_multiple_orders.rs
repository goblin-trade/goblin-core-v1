use crate::{
    goblin_error::GoblinError,
    quantities::{BaseLots, Ticks},
    require,
    types::Address,
};

pub const HANDLE_4_PLACE_MULTIPLE_ORDERS: u8 = 4;
pub const HANDLE_4_HEADER_LEN: usize = core::mem::size_of::<PlaceMultipleOrdersHeader>();
pub const POST_ONLY_ORDER_LEN: usize = core::mem::size_of::<PostOnlyOrder>();

#[repr(C, packed)]
pub struct PlaceMultipleOrdersHeader {
    /// The market base token
    pub base_token: Address,

    /// The market quote token
    pub quote_token: Address,

    /// Whether to fail on cross or whether to amend to amend the price
    pub fail_on_cross: bool,

    /// Whether to skip orders with insufficient funds, or whether to revert the whole TX
    pub skip_on_insufficient_funds: bool,

    /// If no free slots are available at a given price, amend the price away from centre
    /// by this many ticks
    pub tick_offset: u8,

    /// Whether to only use funds in TraderTokenState, or to also transfer in ERC20 tokens
    /// if necessary
    pub use_free_funds: bool,

    /// Number of bids
    pub bids_count: u8,

    /// Number of asks
    pub asks_count: u8,
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

pub fn handle_4_place_multiple_orders(payload: &[u8]) -> Result<usize, GoblinError> {
    require!(
        payload.len() >= HANDLE_4_HEADER_LEN,
        GoblinError::InvalidPayload
    );

    let header = unsafe { &*(payload.as_ptr() as *const PlaceMultipleOrdersHeader) };
    let total_orders = header.bids_count as usize + header.asks_count as usize;
    let orders_len = total_orders * core::mem::size_of::<PostOnlyOrder>();

    let bytes_used = HANDLE_4_HEADER_LEN + orders_len;
    require!(payload.len() >= bytes_used, GoblinError::InvalidPayload);

    let order_bytes = &payload[HANDLE_4_HEADER_LEN..bytes_used];

    handle_4_place_multiple_orders_inner(header, order_bytes).map(|_| bytes_used)
}

/// Externally ensure that order_bytes are of exact length
pub fn handle_4_place_multiple_orders_inner(
    header: &PlaceMultipleOrdersHeader,
    order_bytes: &[u8],
) -> Result<(), GoblinError> {
    // No heap, can't deserialize as dynamic array
    for i in 0..header.bids_count {
        let offset = i as usize * POST_ONLY_ORDER_LEN;
        let _order = unsafe { &*(order_bytes.as_ptr().add(offset) as *const PostOnlyOrder) };

        // TODO insert into book
    }
    Ok(())
}
