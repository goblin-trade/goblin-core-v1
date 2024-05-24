use stylus_sdk::alloy_primitives::{Address, B256};

use crate::{
    parameters::{BASE_LOTS_PER_BASE_UNIT, TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT},
    quantities::{BaseLots, QuoteLots, WrapperU64},
    state::{
        MatchingEngineResponse, MutableBitmap, OrderId, Side, SlotActions, SlotRestingOrder,
        SlotStorage, TickIndices, TraderState, MARKET_STATE_KEY_SEED,
    },
};

use super::{Market, WritableMarket};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct FIFOMarket {
    /// Amount of fees collected from the market in its lifetime, in quote lots.
    collected_quote_lot_fees: QuoteLots,

    /// Amount of unclaimed fees accrued to the market, in quote lots.
    unclaimed_quote_lot_fees: QuoteLots,

    /// The number of active outer indices for bids
    bids_outer_indices: u16,

    /// The number of active outer indices for bids
    asks_outer_indices: u16,
}

const MARKET_SLOT_KEY: [u8; 32] = [
    MARKET_STATE_KEY_SEED,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

impl FIFOMarket {
    pub fn read_from_slot(slot_storage: &SlotStorage) -> Self {
        let slot = slot_storage.sload(&MARKET_SLOT_KEY);

        Self::decode(&slot)
    }

    pub fn decode(slot: &[u8; 32]) -> Self {
        FIFOMarket {
            collected_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[0..8].try_into().unwrap(),
            )),
            unclaimed_quote_lot_fees: QuoteLots::new(u64::from_be_bytes(
                slot[8..16].try_into().unwrap(),
            )),
            bids_outer_indices: u16::from_be_bytes(slot[16..18].try_into().unwrap()),
            asks_outer_indices: u16::from_be_bytes(slot[18..20].try_into().unwrap()),
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        encoded_data[0..8].copy_from_slice(&self.collected_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[8..16].copy_from_slice(&self.unclaimed_quote_lot_fees.as_u64().to_be_bytes());
        encoded_data[16..18].copy_from_slice(&self.bids_outer_indices.to_be_bytes());
        encoded_data[18..20].copy_from_slice(&self.asks_outer_indices.to_be_bytes());

        encoded_data
    }

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage) {
        slot_storage.sstore(&MARKET_SLOT_KEY, &self.encode());
    }

    pub fn outer_index_length(&self, side: Side) -> u16 {
        if side == Side::Bid {
            self.bids_outer_indices
        } else {
            self.asks_outer_indices
        }
    }

    pub fn set_outer_index_length(&mut self, side: Side, value: u16) {
        if side == Side::Bid {
            self.bids_outer_indices = value;
        } else {
            self.asks_outer_indices = value;
        }
    }

    // TODO reduce multiple orders function- needed for cancelations
    // This function doesn't depend on market, it can be moved elsewhere
    fn reduce_order_inner(
        &self,
        remove_index_fn: &mut dyn FnMut(u16),
        trader_state: &mut TraderState,
        order: &mut SlotRestingOrder,
        mutable_bitmap: &mut MutableBitmap,
        trader: Address,
        side: Side,
        order_id: &OrderId,
        size: Option<BaseLots>,
        order_is_expired: bool,
        claim_funds: bool,
    ) -> Option<MatchingEngineResponse> {
        let removed_base_lots = {
            // whether to remove order completely (clear slot), and lots to remove
            let (should_remove_order_from_book, base_lots_to_remove) = {
                // Empty slot- order doesn't exist
                if order.does_not_exist() {
                    return Some(MatchingEngineResponse::default());
                }

                if order.trader_address != trader {
                    return None;
                }

                let base_lots_to_remove = size
                    .map(|s| s.min(order.num_base_lots))
                    .unwrap_or(order.num_base_lots);

                // If the order is tagged as expired, we remove it from the book regardless of the size.
                if order_is_expired {
                    (true, order.num_base_lots)
                } else {
                    (
                        base_lots_to_remove == order.num_base_lots,
                        base_lots_to_remove,
                    )
                }
            };

            let _base_lots_remaining = if should_remove_order_from_book {
                order.clear_order();
                mutable_bitmap.flip(&order_id.resting_order_index);

                // index_list cleared outside

                BaseLots::ZERO
            } else {
                // Reduce order
                order.num_base_lots -= base_lots_to_remove;
                order.num_base_lots
            };

            // EMIT ExpiredOrder / Reduce

            base_lots_to_remove
        };
        // Update trader state
        let (num_quote_lots, num_base_lots) = {
            match side {
                Side::Bid => {
                    let quote_lots = (order_id.price_in_ticks
                        * TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT
                        * removed_base_lots)
                        / BASE_LOTS_PER_BASE_UNIT;
                    trader_state.unlock_quote_lots(quote_lots);

                    (quote_lots, BaseLots::ZERO)
                }
                Side::Ask => {
                    trader_state.unlock_base_lots(removed_base_lots);

                    (QuoteLots::ZERO, removed_base_lots)
                }
            }
        };

        // We don't want to claim funds if an order is removed from the book during a self trade
        // or if the user specifically indicates that they don't want to claim funds.
        if claim_funds {
            self.claim_funds(trader_state, num_quote_lots, num_base_lots)
        } else {
            Some(MatchingEngineResponse::default())
        }
    }

    pub fn cancel_multiple_orders_by_id(
        &self,
        trader_state: &mut TraderState,
        trader: Address,
    ) -> Option<MatchingEngineResponse> {
        None
    }
}

impl Market for FIFOMarket {
    fn get_collected_fee_amount(&self) -> QuoteLots {
        self.collected_quote_lot_fees
    }

    fn get_uncollected_fee_amount(&self) -> QuoteLots {
        self.unclaimed_quote_lot_fees
    }
}

impl WritableMarket for FIFOMarket {
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
    ) -> Option<MatchingEngineResponse> {
        self.reduce_order_inner(
            remove_index_fn,
            trader_state,
            order,
            mutable_bitmap,
            trader,
            side,
            order_id,
            Some(size),
            false,
            claim_funds,
        )
    }

    // TODO move to TraderState
    fn claim_funds(
        &self,
        trader_state: &mut TraderState,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> Option<MatchingEngineResponse> {
        // sequence_number = 0 case removed

        let (quote_lots_received, base_lots_received) = {
            let quote_lots_free = num_quote_lots.min(trader_state.quote_lots_free);
            let base_lots_free = num_base_lots.min(trader_state.base_lots_free);

            // Update and write to slot
            trader_state.quote_lots_free -= quote_lots_free;
            trader_state.base_lots_free -= base_lots_free;

            (quote_lots_free, base_lots_free)
        };

        Some(MatchingEngineResponse::new_withdraw(
            base_lots_received,
            quote_lots_received,
        ))
    }

    fn collect_fees(&mut self) -> QuoteLots {
        let quote_lot_fees = self.unclaimed_quote_lot_fees;

        // Mark as claimed
        self.collected_quote_lot_fees += self.unclaimed_quote_lot_fees;
        self.unclaimed_quote_lot_fees = QuoteLots::ZERO;

        // EMIT MarketEvent::Fee

        quote_lot_fees
    }
}

#[cfg(test)]
mod test {
    use crate::quantities::{QuoteLots, WrapperU64};

    use super::FIFOMarket;

    #[test]
    fn test_encode_and_decode_market_state() {
        let market = FIFOMarket {
            collected_quote_lot_fees: QuoteLots::new(100),
            unclaimed_quote_lot_fees: QuoteLots::new(200),
            bids_outer_indices: 40,
            asks_outer_indices: 10,
        };

        let encoded = market.encode();
        let decoded_market = FIFOMarket::decode(&encoded);

        assert_eq!(market, decoded_market);
    }
}
