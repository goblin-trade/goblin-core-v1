use crate::{
    axis::{
        market::{
            market_locator::hardcoded::HardcodedMarkets, market_marker::MarketMarker,
            market_spec::MarketSpec, MarketReadables, TokenPair,
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
pub trait MarketLocator<TP>
where
    Self: MarketMarker,
    TP: TokenPair + HardcodedMarkets,
    (Self, TP): MarketSpec,
{
    type Locator;

    /// Decode market locator from input args.
    ///
    /// - Hardcoded: Reads and returns just the market index
    /// - Dynamic: Reads all parameters, constructs full MarketAndKey
    fn decode_locator(
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self::Locator, GoblinError>;

    /// Resolve the locator to obtain a reference to the market and its slot key.
    ///
    /// - Hardcoded: Looks up market in static list using index
    /// - Dynamic: Returns reference to the already-constructed market
    fn locate_market(locator: &Self::Locator) -> &MarketReadables<(Self, TP)>;
}
