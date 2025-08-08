use crate::{
    hostio::{self, HostioBuffer},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks},
    state::{SlotKey, SlotState},
    tokens::{ERC20TokenPair, ValidatedTokenPair},
    types::Side,
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
/// We do not use packed because fields are already multiples of 8
#[repr(C)]
pub struct MarketState {
    /// The best bid and best ask price
    /// * Index 0: best bid price
    /// * Index 1: best ask price
    ///
    /// We use an array for branchless lookup
    best_prices: [Ticks; 2],
    _padding: [u8; 24],
}

impl SlotState<MarketKey> for MarketState {}

impl MarketState {
    // pub fn best_price(&self, side: Side) -> Ticks {
    //     self.best_prices[side as usize]
    // }

    pub fn price_limit_reached(&self, order_side: Side, order_price_limit: Ticks) -> bool {
        (order_side == Side::Bid && order_price_limit > self.best_prices[Side::Ask as usize])
            || (order_side == Side::Ask && order_price_limit < self.best_prices[Side::Bid as usize])
    }
}
