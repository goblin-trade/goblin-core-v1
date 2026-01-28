///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
///!
///! However since we use generics, all combinations must be implented. Even hardcoded
///! markets with custom tokens.
use crate::{
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    market::{Hardcoded, HardcodedMarketList, MarketAndKey},
    token::{CustomERC20Data, TokenMarker},
};

pub trait MarketVariant<B, Q>: Clone + Copy
where
    B: TokenMarker,
    Q: TokenMarker,
{
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;

    /// The intermediate representation used to locate a market.
    ///
    /// - Hardcoded: A market index for lookup
    /// - Dynamic: The complete MarketAndKey (acts as its own locator)
    type MarketLocator;

    /// Decode market locator from input args.
    ///
    /// - Hardcoded: Reads and returns just the market index
    /// - Dynamic: Reads all parameters, constructs full MarketAndKey
    fn decode_locator(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::MarketLocator, GoblinError>;

    /// Resolve the locator to obtain a reference to the market and its slot key.
    ///
    /// - Hardcoded: Looks up market in static list using index
    /// - Dynamic: Returns reference to the already-constructed market
    fn locate_market<'a>(
        decoded_market: &'a Self::MarketLocator,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>;
}
