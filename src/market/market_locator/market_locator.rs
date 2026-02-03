use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{HardcodedMarketIndex, HardcodedMarkets, MarketAndKey, MarketVariant},
    token::{CustomERC20Data, TokenMarker},
};

/// The intermediate representation used to locate a market.
///
/// - Hardcoded: A market index for lookup
/// - Dynamic: The complete MarketAndKey (acts as its own locator)
pub trait MarketLocator<M, B, Q>
where
    Self: Sized,
    M: MarketVariant,
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Decode market locator from input args.
    ///
    /// - Hardcoded: Reads and returns just the market index
    /// - Dynamic: Reads all parameters, constructs full MarketAndKey
    fn decode_locator(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self, GoblinError>;

    /// Resolve the locator to obtain a reference to the market and its slot key.
    ///
    /// - Hardcoded: Looks up market in static list using index
    /// - Dynamic: Returns reference to the already-constructed market
    fn locate_market(&self) -> Result<&MarketAndKey<M, B, Q>, GoblinError>
    where
        HardcodedMarketIndex<B, Q>: HardcodedMarkets<B, Q>;
}
