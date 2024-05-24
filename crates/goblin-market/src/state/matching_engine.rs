use stylus_sdk::alloy_primitives::Address;

use crate::quantities::{BaseLots, QuoteLots};

use super::{MarketState, MatchingEngineResponse, SlotActions, SlotStorage, TraderState};

pub struct MatchingEngine<'a> {
    pub slot_storage: &'a mut SlotStorage,
}

impl MatchingEngine<'_> {
    pub fn collect_fees(&mut self) -> QuoteLots {
        // Read
        let mut market = MarketState::read_from_slot(self.slot_storage);

        // Mutate
        let quote_lot_fees = market.unclaimed_quote_lot_fees;

        // Mark as claimed
        market.collected_quote_lot_fees += market.unclaimed_quote_lot_fees;
        market.unclaimed_quote_lot_fees = QuoteLots::ZERO;

        // Write
        market.write_to_slot(self.slot_storage);
        SlotStorage::storage_flush_cache(true);

        quote_lot_fees
    }

    /// Try to claim the given number of lots from a trader's state.
    ///
    /// There is no eviction in Goblin.
    ///
    /// # Parameters
    ///
    /// * `trader` - The trader address
    /// * `num_quote_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U64::MAX to withdraw all.
    /// * `num_base_lots` - Number of lots to withdraw. Pass 0 if none should be withdrawn.
    /// Pass U32::MAX to withdraw all. (max value of base_lots is U32::MAX)
    ///
    pub fn claim_funds(
        &mut self,
        trader: Address,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> MatchingEngineResponse {
        // Read
        let mut trader_state = TraderState::read_from_slot(self.slot_storage, trader);

        // Mutate

        // sequence_number = 0 case removed
        let (quote_lots_received, base_lots_received) = {
            let quote_lots_free = num_quote_lots.min(trader_state.quote_lots_free);
            let base_lots_free = num_base_lots.min(trader_state.base_lots_free);

            // Update and write to slot
            trader_state.quote_lots_free -= quote_lots_free;
            trader_state.base_lots_free -= base_lots_free;

            (quote_lots_free, base_lots_free)
        };

        // Write
        trader_state.write_to_slot(self.slot_storage, trader);
        SlotStorage::storage_flush_cache(true);

        MatchingEngineResponse::new_withdraw(base_lots_received, quote_lots_received)
    }
}
