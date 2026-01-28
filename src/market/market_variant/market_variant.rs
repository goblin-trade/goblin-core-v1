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

    /// The decoded market as read from args
    ///
    /// # Variants
    ///
    /// * Hardcoded: This is simply the MarketIndex, used for looking up the
    /// market from static list.
    ///
    /// * Dynamic: The market params are decoded from args and the key is hashed.
    type DecodedMarket;

    /// Get DecodedMarket from args
    fn decode(
        ctx: &DecodeCtx,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::DecodedMarket, GoblinError>;

    /// Obtain reference to the market and key
    ///
    /// # Variants
    ///
    /// * Hardcoded: Use market key to obtain static lifetime market,
    /// then cast it to local lifetime.
    ///
    /// * Dynamic: DecodedMarket is MarketAndKey. Obtain a reference.
    ///
    fn market_and_key_ref<'a>(
        decoded_market: &'a Self::DecodedMarket,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>;
}
