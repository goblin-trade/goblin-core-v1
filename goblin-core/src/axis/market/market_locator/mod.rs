pub mod dynamic;
pub mod hardcoded;
pub mod market_index;

pub use hardcoded::*;
pub use market_index::*;

use crate::{
    axis::token::TokenDataTriple, axis_helpers::TokenPair, goblin_error::GoblinError,
    input_processor::CodecBounds, market::MarketReadables,
};

/// The intermediate representation used to locate a market.
///
/// - Hardcoded: A market index, resolved against the static market list
/// - Dynamic: The full decoded `CommonMarket`, resolved by hashing its token
///   addresses into a slot key
///
/// The locator is exactly the value that travels on the wire, so it must be a
/// plain codec type. It deliberately does **not** carry a slot key: a hash sent
/// by the caller cannot be trusted. `locate_market` derives the key inside the
/// contract instead.
pub trait MarketLocator<TP: TokenPair + HardcodedMarketList> {
    /// Wire representation of the locator, decoded and encoded with the default
    /// codec context.
    type Locator: CodecBounds;

    /// Resolve the locator to a market and its slot key.
    ///
    /// - Hardcoded: maps the index into the static hardcoded market list
    /// - Dynamic: derives the slot key by hashing the decoded market
    fn locate_market(
        locator: &Self::Locator,
        token_data_triple: &TokenDataTriple,
    ) -> Result<MarketReadables<TP>, GoblinError>;
}
