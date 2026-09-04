use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoblinSdkError {
    CustomTokenLimitExceeded(usize),
    MarketCountLimitExceeded(usize, usize),
    OuterBitmapLimitExceeded(usize),
    InnerBitmapLimitExceeded(usize),
    MakeUpdateLimitExceeded(usize),
    InvalidTokenPair {
        market: &'static str,
        base: &'static str,
        quote: &'static str,
    },
    ZeroTakeLots,
    TakeLotsOverflow(u64),
    MakeLotsOverflow(u64),
    InvalidHexAddress(alloc::string::String),
    HexDecodeError(alloc::string::String),
    InvalidMarketIndex(usize, u8),
    CustomTokenIndexOutOfRange(u8, usize),
    InvalidLotSize(u64),
    ZeroLimit,
}

impl fmt::Display for GoblinSdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomTokenLimitExceeded(count) => {
                write!(
                    f,
                    "Custom ERC20 token count exceeded maximum of 7, got {}",
                    count
                )
            }
            Self::MarketCountLimitExceeded(spec, count) => {
                write!(
                    f,
                    "Market count for spec {} exceeded maximum of 15, got {}",
                    spec, count
                )
            }
            Self::OuterBitmapLimitExceeded(count) => {
                write!(
                    f,
                    "Outer bitmap count exceeded maximum of 3 for market, got {}",
                    count
                )
            }
            Self::InnerBitmapLimitExceeded(count) => {
                write!(
                    f,
                    "Inner bitmap count exceeded maximum of 255 for outer bitmap, got {}",
                    count
                )
            }
            Self::MakeUpdateLimitExceeded(count) => {
                write!(
                    f,
                    "Make update count exceeded maximum of 255 for inner bitmap, got {}",
                    count
                )
            }
            Self::InvalidTokenPair {
                market,
                base,
                quote,
            } => {
                write!(
                    f,
                    "Invalid token pair combination: market={}, base={}, quote={}",
                    market, base, quote
                )
            }
            Self::ZeroTakeLots => write!(f, "Invalid take lots: must be greater than 0"),
            Self::TakeLotsOverflow(lots) => {
                write!(
                    f,
                    "Take order lots overflowed u64 when shifted by 2: {}",
                    lots
                )
            }
            Self::MakeLotsOverflow(lots) => {
                write!(
                    f,
                    "Make order base lots overflowed u64 when shifted by 2: {}",
                    lots
                )
            }
            Self::InvalidHexAddress(addr) => write!(f, "Invalid hex address: {}", addr),
            Self::HexDecodeError(err) => write!(f, "Invalid hex string: {}", err),
            Self::InvalidMarketIndex(spec, idx) => {
                write!(f, "Invalid market index for spec {}: {}", spec, idx)
            }
            Self::CustomTokenIndexOutOfRange(idx, total) => {
                write!(
                    f,
                    "Custom token index {} out of range (total custom tokens: {})",
                    idx, total
                )
            }
            Self::InvalidLotSize(size) => {
                write!(f, "Lot size must be a divisor of ATOMS_PER_UNIT: {}", size)
            }
            Self::ZeroLimit => write!(f, "Limit price/position must be greater than 0"),
        }
    }
}

impl core::error::Error for GoblinSdkError {}
