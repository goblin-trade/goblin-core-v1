use crate::{
    markets::LotSizePair,
    quantities::QuoteLotsPerBaseUnitPerTick,
    tokens::{DynamicTokenIndex, HardcodedIndex, TokenPair},
};

pub enum Market {
    Hardcoded(HardcodedMarket),
    Custom(CustomMarket),
}

pub struct HardcodedMarket {
    pub common_market: CommonMarket<HardcodedIndex>,
    pub keccak_hash: [u8; 32],
}

pub type CustomMarket = CommonMarket<DynamicTokenIndex>;

// We need 3 market types corresponding to the 3 varieties of TokenPair
pub struct CommonMarket<T> {
    pub token_pair: TokenPair<T>,
    pub lot_size_pair: LotSizePair,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}
