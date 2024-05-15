use stylus_sdk::alloy_primitives::Address;

use crate::quantities::{BaseLots, QuoteLots, WrapperU64};

use super::{SlotActions, SlotKey, SlotStorage, TRADER_STATE_KEY_SEED};

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
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct TraderState {
    pub quote_lots_locked: QuoteLots,
    pub quote_lots_free: QuoteLots,
    pub base_lots_locked: BaseLots,
    pub base_lots_free: BaseLots,
}

impl TraderState {
    pub fn read_from_slot(slot_storage: &SlotStorage, trader_id: TraderId) -> Self {
        let slot_key = trader_id.get_key();
        let slot = slot_storage.sload(&slot_key);

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

    pub fn write_to_slot(&self, slot_storage: &mut SlotStorage, trader_id: TraderId) {
        slot_storage.sstore(&trader_id.get_key(), &self.encode());
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

    #[inline(always)]
    pub(crate) fn lock_quote_lots(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_locked += quote_lots;
    }

    #[inline(always)]
    pub(crate) fn lock_base_lots(&mut self, base_lots: BaseLots) {
        self.base_lots_locked += base_lots;
    }

    #[inline(always)]
    pub(crate) fn use_free_quote_lots(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_free -= quote_lots;
    }

    #[inline(always)]
    pub(crate) fn use_free_base_lots(&mut self, base_lots: BaseLots) {
        self.base_lots_free -= base_lots;
    }

    #[inline(always)]
    pub(crate) fn deposit_free_quote_lots(&mut self, quote_lots: QuoteLots) {
        self.quote_lots_free += quote_lots;
    }

    #[inline(always)]
    pub(crate) fn deposit_free_base_lots(&mut self, base_lots: BaseLots) {
        self.base_lots_free += base_lots;
    }
}

#[cfg(test)]
mod test {
    use crate::quantities::{BaseLots, QuoteLots, WrapperU64};

    use super::TraderState;

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
