use crate::{
    axis::{
        market::{
            market_marker::hardcoded::{
                hardcoded_market_index::HardcodedMarketIndex, hardcoded_markets::HardcodedMarkets,
            },
            market_spec::MarketSpec,
            MarketReadables,
        },
        token::token_reader::TokenDataTriple,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
};

/// The intermediate representation used to locate a market.
///
/// - Hardcoded: A market index for lookup
/// - Dynamic: The complete MarketAndKey (acts as its own locator)
pub trait MarketLocator<MS: MarketSpec>
where
    Self: Sized,
{
    /// Decode market locator from input args.
    ///
    /// - Hardcoded: Reads and returns just the market index
    /// - Dynamic: Reads all parameters, constructs full MarketAndKey
    fn decode_locator(
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError>;

    /// Resolve the locator to obtain a reference to the market and its slot key.
    ///
    /// - Hardcoded: Looks up market in static list using index
    /// - Dynamic: Returns reference to the already-constructed market
    fn locate_market(&self) -> Result<&MarketReadables<MS>, GoblinError>
    where
        HardcodedMarketIndex<MS::Pair>: HardcodedMarkets<MS::Pair>;
}
