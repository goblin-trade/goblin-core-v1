use crate::{
    hostio::{self, HostioBuffer},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks},
    state::{SlotKey, SlotState},
    tokens::{ERC20TokenPair, ValidatedTokenPair},
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
        token_pair: &ValidatedTokenPair,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        match token_pair {
            ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                base_token,
                quote_token,
            }) => {
                let mut bytes = [0u8; (1 + 2 * 20 + 3 * 8)];
                bytes[0] = token_pair.discriminator();
                bytes[1..21].copy_from_slice(base_token.address());
                bytes[21..41].copy_from_slice(quote_token.address());
                bytes[41..49].copy_from_slice(&base_lot_size.0.to_le_bytes());
                bytes[49..57].copy_from_slice(&quote_lot_size.0.to_le_bytes());
                bytes[57..65].copy_from_slice(&tick_size.0.to_le_bytes());

                let hash = hostio::native_keccak256(bytes.as_slice());

                Self { hash }
            }
            ValidatedTokenPair::ETHERC20(erc20_token)
            | ValidatedTokenPair::ERC20ETH(erc20_token) => {
                let mut bytes = [0u8; (1 + 20 + 3 * 8)];
                bytes[0] = token_pair.discriminator();
                bytes[1..21].copy_from_slice(erc20_token.address());
                bytes[21..29].copy_from_slice(&base_lot_size.0.to_le_bytes());
                bytes[29..37].copy_from_slice(&quote_lot_size.0.to_le_bytes());
                bytes[37..45].copy_from_slice(&tick_size.0.to_le_bytes());

                let hash = hostio::native_keccak256(bytes.as_slice());

                Self { hash }
            }
        }
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
