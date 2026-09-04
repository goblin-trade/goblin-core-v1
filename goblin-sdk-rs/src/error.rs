use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GoblinSdkError {
    #[error("Custom ERC20 token count exceeded maximum of 7, got {0}")]
    CustomTokenLimitExceeded(usize),

    #[error("Market count for spec {0} exceeded maximum of 15, got {1}")]
    MarketCountLimitExceeded(usize, usize),

    #[error("Outer bitmap count exceeded maximum of 3 for market, got {0}")]
    OuterBitmapLimitExceeded(usize),

    #[error("Inner bitmap count exceeded maximum of 255 for outer bitmap, got {0}")]
    InnerBitmapLimitExceeded(usize),

    #[error("Make update count exceeded maximum of 255 for inner bitmap, got {0}")]
    MakeUpdateLimitExceeded(usize),

    #[error("Invalid token pair combination: market={market:?}, base={base:?}, quote={quote:?}")]
    InvalidTokenPair {
        market: &'static str,
        base: &'static str,
        quote: &'static str,
    },

    #[error("Invalid take lots: must be greater than 0")]
    ZeroTakeLots,

    #[error("Take order lots overflowed u64 when shifted by 2: {0}")]
    TakeLotsOverflow(u64),

    #[error("Make order base lots overflowed u64 when shifted by 2: {0}")]
    MakeLotsOverflow(u64),

    #[error("Invalid hex address: {0}")]
    InvalidHexAddress(String),

    #[error("Invalid hex string: {0}")]
    HexDecodeError(String),

    #[error("Invalid market index for spec {0}: {1}")]
    InvalidMarketIndex(usize, u8),

    #[error("Custom token index {0} out of range (total custom tokens: {1})")]
    CustomTokenIndexOutOfRange(u8, usize),

    #[error("Lot size must be a divisor of ATOMS_PER_UNIT (1,000,000): {0}")]
    InvalidLotSize(u64),

    #[error("Limit price/position must be greater than 0")]
    ZeroLimit,
}
