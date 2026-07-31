use crate::{
    axis::{
        market::{
            market_locator::{hardcoded::HardcodedMarkets, MarketIndex},
            market_spec::MarketSpec,
            Hardcoded, MarketReadables,
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
///
/// We perform two operations to avoid copying static hardcoded market data
/// onto the stack
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
    fn locate_market(&self) -> &MarketReadables<MS>
    where
        MarketIndex<(Hardcoded, MS::Pair)>: HardcodedMarkets<MS::Pair>;
}
