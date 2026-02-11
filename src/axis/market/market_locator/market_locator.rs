use crate::{
    axis::{
        market::{
            market_marker::{
                hardcoded::{
                    hardcoded_market_index::HardcodedMarketIndex,
                    hardcoded_markets::HardcodedMarkets,
                },
                MarketMarker,
            },
            MarketAndKey,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
};

/// The intermediate representation used to locate a market.
///
/// - Hardcoded: A market index for lookup
/// - Dynamic: The complete MarketAndKey (acts as its own locator)
pub trait MarketLocator<M, B, Q>
where
    Self: Sized,
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Decode market locator from input args.
    ///
    /// - Hardcoded: Reads and returns just the market index
    /// - Dynamic: Reads all parameters, constructs full MarketAndKey
    fn decode_locator<'a>(
        ctx: &DecodeCtx,
        erc20_list: M::ERC20List<'a>,
    ) -> Result<Self, GoblinError>;

    /// Resolve the locator to obtain a reference to the market and its slot key.
    ///
    /// - Hardcoded: Looks up market in static list using index
    /// - Dynamic: Returns reference to the already-constructed market
    fn locate_market(&self) -> Result<&MarketAndKey<M, B, Q>, GoblinError>
    where
        HardcodedMarketIndex<B, Q>: HardcodedMarkets<B, Q>;
}
