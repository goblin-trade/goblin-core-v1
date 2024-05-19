use stylus_sdk::alloy_primitives::Address;

use crate::{
    quantities::{BaseLots, QuoteLots},
    state::{
        BitmapGroup, MatchingEngineResponse, OrderId, Side, SlotRestingOrder, SlotStorage,
        TraderId, TraderState,
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

    fn get_sequence_number(&self) -> u64;
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
        trader_state: &mut TraderState,
        order: &mut SlotRestingOrder,
        bitmap_group: &mut BitmapGroup,
        trader: Address,
        side: Side,
        order_id: &OrderId,
        size: BaseLots,
        claim_funds: bool,
    ) -> Option<MatchingEngineResponse>;

    /// Try to claim the given number of lots from a trader's state.
    ///
    /// There is no eviction in Goblin.
    ///
    /// # Parameters
    ///
    /// * `slot_storage`
    /// * `trader` - The trader address
    /// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    /// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U32::MAX to withdraw all. (max value of base_lots is U32::MAX)
    ///
    fn claim_funds(
        &self,
        // slot_storage: &mut SlotStorage,
        trader_state: &mut TraderState,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> Option<MatchingEngineResponse>;

    /// Collect protocol fees. Returns the amount of quote lots to claim
    fn collect_fees(&mut self) -> QuoteLots;
}
