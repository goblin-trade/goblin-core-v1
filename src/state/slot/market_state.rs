use crate::{
    hostio::{hostio_native_keccak256, HostioBuffer},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks},
    state::{SlotKey, SlotState},
    types::Address,
};

pub struct MarketKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKey for MarketKey {
    const DISCRIMINATOR: u8 = 3;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

impl MarketKey {
    pub fn new(
        base_token: &Address,
        quote_token: &Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        let mut bytes = [0u8; (1 + 2 * 20 + 3 * 8)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..21].copy_from_slice(base_token);
        bytes[21..41].copy_from_slice(quote_token);
        bytes[41..49].copy_from_slice(&base_lot_size.0.to_le_bytes());
        bytes[49..57].copy_from_slice(&quote_lot_size.0.to_le_bytes());
        bytes[57..65].copy_from_slice(&tick_size.0.to_le_bytes());

        let hash = unsafe { hostio_native_keccak256(bytes.as_slice()) };

        Self { hash }
    }
}

/// The market state stored in a slot
#[repr(C, packed)]
pub struct MarketState {
    /// The number of active outer indices for bids
    pub bids_outer_indices: u16,

    /// The number of active outer indices for asks
    pub asks_outer_indices: u16,

    /// Price of the highest bid
    pub best_bid_price: Ticks,

    /// The lowest ask
    pub best_ask_price: Ticks,

    _padding: [u8; 20],
}

impl SlotState<MarketKey> for MarketState {}
