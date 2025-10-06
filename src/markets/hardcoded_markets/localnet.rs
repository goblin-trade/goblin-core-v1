use crate::{
    markets::{CommonMarket, HardcodedMarket},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::{HardcodedToken, TokenIndex, ERC20, ETH},
    types::Pair,
};

// ───────────────────────────────────────────────
// Hardcoded ETH–ERC20 markets
// ───────────────────────────────────────────────

pub const HARDCODED_MARKETS_ETH_BASE_ERC20_QUOTE: [HardcodedMarket<Pair<ETH, ERC20>>; 1] =
    [HardcodedMarket {
        common: CommonMarket {
            token_pair: TokenIndex::<HardcodedToken>::new(0),
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(100),
                quote: QuoteLotsPerQuoteUnit::new(1000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: [0u8; 32],
    }];

// ───────────────────────────────────────────────
// Hardcoded ERC20–ETH markets
// ───────────────────────────────────────────────

pub const HARDCODED_MARKETS_ERC20_BASE_ETH_QUOTE: [HardcodedMarket<Pair<ERC20, ETH>>; 1] =
    [HardcodedMarket {
        common: CommonMarket {
            token_pair: TokenIndex::<HardcodedToken>::new(1),
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(200),
                quote: QuoteLotsPerQuoteUnit::new(2000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: [0u8; 32],
    }];

// ───────────────────────────────────────────────
// Hardcoded ERC20–ERC20 markets
// ───────────────────────────────────────────────

pub const HARDCODED_MARKETS_ERC20_BASE_ERC20_QUOTE: [HardcodedMarket<Pair<ERC20, ERC20>>; 1] =
    [HardcodedMarket {
        common: CommonMarket {
            token_pair: Pair {
                base: TokenIndex::<HardcodedToken>::new(0),
                quote: TokenIndex::<HardcodedToken>::new(1),
            },
            lot_size_pair: Pair {
                base: BaseLotsPerBaseUnit::new(300),
                quote: QuoteLotsPerQuoteUnit::new(3000),
            },
            tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
        },
        keccak_hash: [0u8; 32],
    }];
