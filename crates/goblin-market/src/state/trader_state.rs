use stylus_sdk::alloy_primitives::Address;

use crate::{
    program::{compute_quote_lots, types::matching_engine_response::MatchingEngineResponse},
    quantities::{BaseLots, QuoteLots, Ticks, WrapperU64},
};

use super::{
    order::resting_order::{RestingOrder, SlotRestingOrder},
    ArbContext, ContextActions, Side, SlotKey, TRADER_STATE_KEY_SEED,
};

pub type TraderId = Address;

impl SlotKey for TraderId {
    fn get_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];

        key[0] = TRADER_STATE_KEY_SEED;
        key[1..21].copy_from_slice(self.as_slice());

        key
    }
}

#[repr(C)]
#[derive(Default, Debug, PartialEq)]
pub struct TraderState {
    pub quote_lots_locked: QuoteLots,
    pub quote_lots_free: QuoteLots,
    pub base_lots_locked: BaseLots,
    pub base_lots_free: BaseLots,
}

impl TraderState {
    pub fn read_from_slot(ctx: &ArbContext, trader_id: TraderId) -> Self {
        let slot_key = trader_id.get_key();
        let slot = ctx.sload(&slot_key);

        Self::decode(&slot)
    }

    pub fn decode(slot: &[u8; 32]) -> Self {
        TraderState {
            quote_lots_locked: QuoteLots::new(u64::from_be_bytes(slot[0..8].try_into().unwrap())),
            quote_lots_free: QuoteLots::new(u64::from_be_bytes(slot[8..16].try_into().unwrap())),
            base_lots_locked: BaseLots::new(u64::from_be_bytes(slot[16..24].try_into().unwrap())),
            base_lots_free: BaseLots::new(u64::from_be_bytes(slot[24..32].try_into().unwrap())),
        }
    }

    pub fn encode(&self) -> [u8; 32] {
        let mut encoded_data = [0u8; 32];

        encoded_data[0..8].copy_from_slice(&self.quote_lots_locked.as_u64().to_be_bytes());
        encoded_data[8..16].copy_from_slice(&self.quote_lots_free.as_u64().to_be_bytes());
        encoded_data[16..24].copy_from_slice(&self.base_lots_locked.as_u64().to_be_bytes());
        encoded_data[24..32].copy_from_slice(&self.base_lots_free.as_u64().to_be_bytes());

        encoded_data
    }

    pub fn write_to_slot(&self, ctx: &mut ArbContext, trader_id: TraderId) {
        ctx.sstore(&trader_id.get_key(), &self.encode());
    }

    /// Claim a specified number of quote and base lots from the trader state. The
    /// trader state balances are deduced. The amount claimed may be less if
    /// the available free funds are less than the requested funds.
    ///
    /// # Arguments
    ///
    /// * `num_quote_lots` - The number of quote lots to be claimed.
    /// * `num_base_lots` - The number of base lots to be claimed.
    ///
    /// # Returns
    ///
    /// * `MatchingEngineResponse` - A response containing the actual number of base and
    ///   quote lots claimed, which may be less than the requested amount if the available
    ///   free lots are smaller.
    ///
    pub fn claim_funds(
        &mut self,
        num_quote_lots: QuoteLots,
        num_base_lots: BaseLots,
    ) -> MatchingEngineResponse {
        let quote_lots_received = num_quote_lots.min(self.quote_lots_free);
        let base_lots_received = num_base_lots.min(self.base_lots_free);

        self.quote_lots_free -= quote_lots_received;
        self.base_lots_free -= base_lots_received;

        MatchingEngineResponse::new_withdraw(base_lots_received, quote_lots_received)
    }

    /// Credits output lots into free lots of the trader state and clears the output lots in
    /// the matching engine response
    pub fn deposit_output_into_free_lots(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        side: Side,
    ) {
        match side {
            Side::Bid => {
                self.deposit_free_base_lots(matching_engine_response.num_base_lots_out);
                matching_engine_response.num_base_lots_out = BaseLots::ZERO;
            }
            Side::Ask => {
                self.deposit_free_quote_lots(matching_engine_response.num_quote_lots_out);
                matching_engine_response.num_quote_lots_out = QuoteLots::ZERO;
            }
        }
    }

    /// Deposit free quote lots in the trader state
    #[inline(always)]
    pub(crate) fn deposit_free_quote_lots(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_free += quote_lots;
    }

    /// Deposit free base lots in the trader state
    #[inline(always)]
    pub(crate) fn deposit_free_base_lots(&mut self, base_lots: BaseLots) {
        self.base_lots_free += base_lots;
    }

    #[inline(always)]
    pub(crate) fn unlock_quote_lots(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_locked -= quote_lots;
        self.quote_lots_free += quote_lots;
    }

    #[inline(always)]
    pub(crate) fn unlock_base_lots(&mut self, base_lots: BaseLots) {
        self.base_lots_locked -= base_lots;
        self.base_lots_free += base_lots;
    }

    #[inline(always)]
    pub(crate) fn process_limit_sell(
        &mut self,
        base_lots_removed: BaseLots,
        quote_lots_received: QuoteLots,
    ) {
        self.base_lots_locked -= base_lots_removed;
        self.quote_lots_free += quote_lots_received;
    }

    #[inline(always)]
    pub(crate) fn process_limit_buy(
        &mut self,
        quote_lots_removed: QuoteLots,
        base_lots_received: BaseLots,
    ) {
        self.quote_lots_locked -= quote_lots_removed;
        self.base_lots_free += base_lots_received;
    }

    /// Locks the given number of quote lots in the trader state and posts them in
    /// the matching engine response
    ///
    /// # Arguments
    ///
    /// * `matching_engine_response`
    /// * `quote_lots` - Number of quote lots to lock / post.
    #[inline(always)]
    pub(crate) fn lock_quote_lots(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        quote_lots: QuoteLots,
    ) {
        self.lock_quote_lots_inner(quote_lots);
        matching_engine_response.post_quote_lots(quote_lots);
    }

    /// Locks the given number of base lots in the trader state and posts them in
    /// the matching engine response
    ///
    /// # Arguments
    ///
    /// * `matching_engine_response`
    /// * `base_lots` - Number of base lots to lock / post.
    #[inline(always)]
    pub(crate) fn lock_base_lots(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        base_lots: BaseLots,
    ) {
        self.lock_base_lots_inner(base_lots);
        matching_engine_response.post_base_lots(base_lots);
    }

    /// Deduct free tokens for a matched take order and generate
    /// a matching engine response
    ///
    /// # Arguments
    ///
    /// * `side` - Whether bid or ask
    /// * `matched_quote_lots` - Number of quote lots matched
    /// * `matched_base_lots` - Number of base lots matched
    pub fn take_order(
        &mut self,
        side: Side,
        matched_quote_lots: QuoteLots,
        matched_base_lots: BaseLots,
    ) -> MatchingEngineResponse {
        match side {
            // Quote lots in, base lots out
            Side::Bid => {
                let quote_lots_free_to_use = self.quote_lots_free.min(matched_quote_lots);
                self.use_free_quote_lots_inner(quote_lots_free_to_use);

                MatchingEngineResponse::new_from_buy(
                    matched_quote_lots,
                    matched_base_lots,
                    quote_lots_free_to_use,
                )
            }
            // Base lots in, quote lots out
            Side::Ask => {
                let base_lots_free_to_use = self.base_lots_free.min(matched_base_lots);
                self.use_free_base_lots_inner(base_lots_free_to_use);

                MatchingEngineResponse::new_from_sell(
                    matched_base_lots,
                    matched_quote_lots,
                    base_lots_free_to_use,
                )
            }
        }
    }

    /// Lock up lots and use available free lots to post a resting order
    ///
    /// If deposited free lots are insufficient to cover the order, the remainder
    /// amount will be transferred by an ERC20 transfer later. The remainder amount
    /// is tracked by MatchingEngineResponse.
    ///
    /// # Arguments
    ///
    /// * `matching_engine_response`
    /// * `side`
    /// * `price_in_ticks` - Price where order should be placed
    /// * `num_base_lots` - Side of the order
    ///
    /// # Updates
    ///
    /// * `self`- Adds to locked lots, deducts available free lots
    /// * `matching_engine_response` - Adds to the lots posted (locked) on the book
    /// free lots used.
    pub fn make_order(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        side: Side,
        price_in_ticks: Ticks,
        num_base_lots: BaseLots,
    ) {
        match side {
            Side::Bid => {
                // Lots needed to place order
                let quote_lots_to_lock = compute_quote_lots(price_in_ticks, num_base_lots);
                // Deposited lots available with TraderState
                let quote_lots_free_to_use = quote_lots_to_lock.min(self.quote_lots_free);

                self.lock_quote_lots(matching_engine_response, quote_lots_to_lock);
                self.use_free_quote_lots(matching_engine_response, quote_lots_free_to_use);
            }
            Side::Ask => {
                let base_lots_free_to_use = num_base_lots.min(self.base_lots_free);

                self.lock_base_lots(matching_engine_response, num_base_lots);
                self.use_free_base_lots(matching_engine_response, base_lots_free_to_use);
            }
        }
    }

    /// Uses up free quote lots from the trader state. This used amount is tracked
    /// in Matching Engine Response
    ///
    /// # Arguments
    ///
    /// * `matching_engine_response`
    /// * `quote_lots`
    #[inline(always)]
    pub(crate) fn use_free_quote_lots(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        quote_lots: QuoteLots,
    ) {
        self.use_free_quote_lots_inner(quote_lots);
        matching_engine_response.use_free_quote_lots(quote_lots);
    }

    /// Uses up free base lots from the trader state. This used amount is tracked in Matching Engine Response
    ///
    /// # Arguments
    ///
    /// * `matching_engine_response`
    /// * `base_lots`
    #[inline(always)]
    pub(crate) fn use_free_base_lots(
        &mut self,
        matching_engine_response: &mut MatchingEngineResponse,
        base_lots: BaseLots,
    ) {
        self.use_free_base_lots_inner(base_lots);
        matching_engine_response.use_free_base_lots(base_lots);
    }

    /// Lock up quote lots
    #[inline(always)]
    fn lock_quote_lots_inner(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_locked += quote_lots;
    }

    /// Lock up base lots
    #[inline(always)]
    fn lock_base_lots_inner(&mut self, base_lots: BaseLots) {
        self.base_lots_locked += base_lots;
    }

    /// Use up free quote lots
    #[inline(always)]
    fn use_free_quote_lots_inner(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_free -= quote_lots;
    }

    /// Use up free base lots
    #[inline(always)]
    fn use_free_base_lots_inner(&mut self, base_lots: BaseLots) {
        self.base_lots_free -= base_lots;
    }

    /// Cancel a resting order, credit funds to the trader state and
    /// generate a matching engine response.
    ///
    /// Externally ensure that the resting order is removed from the book
    /// using the sequential remover. This function clears base lots from
    /// the resting order but does not remove it from the orderbook.
    ///
    /// # Arguments
    ///
    /// * `resting_order`
    /// * `side`
    /// * `price_in_ticks`
    /// * `claim_funds` - Whether funds should be claimed via token transfer
    /// or be credited to the trader state.
    ///
    pub fn cancel_order_and_claim_funds(
        &mut self,
        resting_order: &mut SlotRestingOrder,
        side: Side,
        price_in_ticks: Ticks,
        claim_funds: bool,
    ) -> MatchingEngineResponse {
        // Deduct all lots
        let base_lots_to_remove = resting_order.num_base_lots;
        resting_order.num_base_lots = BaseLots::ZERO;

        // EMIT ExpiredOrder / Reduce

        // Free up tokens from trader state
        let (num_quote_lots, num_base_lots) = {
            match side {
                Side::Bid => {
                    // A bid order consists of locked up 'quote tokens' bidding to buy the base token.
                    // Quote tokens are released on reducing the order.
                    let quote_lots = compute_quote_lots(price_in_ticks, base_lots_to_remove);

                    self.unlock_quote_lots(quote_lots);
                    (quote_lots, BaseLots::ZERO)
                }
                Side::Ask => {
                    // A bid order consists of locked up 'base tokens' bidding to sell the base token
                    // in exchange of the quote token.
                    // Base tokens are released on reducing the order.
                    self.unlock_base_lots(base_lots_to_remove);
                    (QuoteLots::ZERO, base_lots_to_remove)
                }
            }
        };

        if claim_funds {
            self.claim_funds(num_quote_lots, num_base_lots)
        } else {
            // No claim case
            // The order is reduced but no funds will be claimed
            MatchingEngineResponse::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quantities::{BaseLots, QuoteLots, WrapperU64};

    use super::TraderState;

    mod test_encode {
        use super::*;
        #[test]
        fn test_encode_and_decode_trader_state() {
            let trader_state = TraderState {
                quote_lots_locked: QuoteLots::new(100),
                quote_lots_free: QuoteLots::new(200),
                base_lots_locked: BaseLots::new(300),
                base_lots_free: BaseLots::new(400),
            };

            let encoded = trader_state.encode();

            let decoded_trader_state = TraderState::decode(&encoded);

            assert_eq!(trader_state, decoded_trader_state);
        }
    }

    mod test_claim_funds {
        use crate::program::types::matching_engine_response::MatchingEngineResponse;

        use super::*;

        #[test]
        fn test_claim_when_no_free_tokens() {
            let mut trader_state = TraderState {
                quote_lots_locked: QuoteLots::ZERO,
                quote_lots_free: QuoteLots::ZERO,
                base_lots_locked: BaseLots::ZERO,
                base_lots_free: BaseLots::ZERO,
            };

            let num_quote_lots = QuoteLots::ZERO;
            let num_base_lots = BaseLots::ONE;

            let response = trader_state.claim_funds(num_quote_lots, num_base_lots);

            assert_eq!(trader_state, TraderState::default());
            assert_eq!(response, MatchingEngineResponse::default());
        }

        #[test]
        fn test_claim_base_lots() {
            let mut trader_state = TraderState {
                quote_lots_locked: QuoteLots::ZERO,
                quote_lots_free: QuoteLots::ZERO,
                base_lots_locked: BaseLots::ZERO,
                base_lots_free: BaseLots::new(2),
            };

            let num_quote_lots = QuoteLots::ZERO;
            let num_base_lots = BaseLots::ONE;

            let response = trader_state.claim_funds(num_quote_lots, num_base_lots);

            assert_eq!(
                trader_state,
                TraderState {
                    quote_lots_locked: QuoteLots::ZERO,
                    quote_lots_free: QuoteLots::ZERO,
                    base_lots_locked: BaseLots::ZERO,
                    base_lots_free: BaseLots::new(1),
                }
            );

            assert_eq!(
                response,
                MatchingEngineResponse {
                    num_quote_lots_in: QuoteLots::ZERO,
                    num_base_lots_in: BaseLots::ZERO,
                    num_quote_lots_out: QuoteLots::ZERO,
                    num_base_lots_out: BaseLots::new(1),
                    num_quote_lots_posted: QuoteLots::ZERO,
                    num_base_lots_posted: BaseLots::ZERO,
                    num_free_quote_lots_used: QuoteLots::ZERO,
                    num_free_base_lots_used: BaseLots::ZERO
                }
            );
        }
    }
}
