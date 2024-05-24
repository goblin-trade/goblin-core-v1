use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::{BaseLots, QuoteLots},
    state::{
        BitmapGroup, MatchingEngineResponse, MutableBitmap, OrderId, Side, SlotRestingOrder,
        SlotStorage, TraderId, TraderState,
    },
};

pub trait RestingOrder {
    fn size(&self) -> u64;
    fn last_valid_block(&self) -> Option<u32>;
    fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32>;
    // fn is_expired(&self, current_slot: u32, current_unix_timestamp_in_seconds: u32) -> bool;
}

pub trait Market {
    fn get_collected_fee_amount(&self) -> QuoteLots;

    fn get_uncollected_fee_amount(&self) -> QuoteLots;
}

pub trait WritableMarket {
    /// Try to reduce a resting order
    ///
    /// # Arguments
    ///
    /// * `trader_state`
    /// *  `order` - Resting order at slot
    /// * `bitmap_group` - Bitmap group for the given outer index
    /// * `trader` - Reduce order for this trader
    /// * `side` - Order size in BaseLots
    /// * `order_id` - Order ID, i.e. tick and resting order index
    /// * `size` - Reduce by this many base lots
    /// * `recipient` - Optional. If provided, withdraw freed funds to this address.
    ///
    fn reduce_order(
        &self,
        remove_index_fn: &mut dyn FnMut(u16),
        trader_state: &mut TraderState,
        order: &mut SlotRestingOrder,
        mutable_bitmap: &mut MutableBitmap,
        trader: Address,
        side: Side,
        order_id: &OrderId,
        size: BaseLots,
        claim_funds: bool,
    ) -> Option<MatchingEngineResponse>;
}
