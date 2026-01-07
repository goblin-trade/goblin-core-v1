///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either dynamic or custom
use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{Dynamic, Hardcoded, HardcodedMarketList, MarketAndKey},
    state::MarketState,
    token::{CustomToken, TokenMarker},
    types::Address,
};

pub trait MarketVariant: Clone + Copy {
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;

    /// The token index type for this market variant
    ///
    /// Hardcoded variant uses hardcoded token index whereas the dynamic
    /// variant uses an enum of hardcoded and custom token index
    type TokenIndex: Clone + Copy;

    /// The decoded market as read from args
    ///
    /// # Variants
    ///
    /// * Hardcoded: This is simply the MarketIndex, used for looking up the
    /// market from static list.
    ///
    /// * Dynamic: The market params are decoded from args and the key is hashed.
    type DecodedMarket<B: TokenMarker, Q: TokenMarker>;

    fn token_index_to_address_inner(
        index: Self::TokenIndex,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Address, GoblinError>;

    /// Get DecodedMarket from args
    fn decode<'a, B, Q>(
        ctx: &'a DecodeCtx<'a>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::TokenIndex<Dynamic>: Decodable<'a>,
        Q::TokenIndex<Dynamic>: Decodable<'a>;
    // MarketState<Dynamic, B, Q>: DynamicMarketHasher<B, Q>;

    /// Obtain reference to the market and key
    ///
    /// # Variants
    ///
    /// * Hardcoded: Use market key to obtain static lifetime market,
    /// then cast it to local lifetime.
    ///
    /// * Dynamic: DecodedMarket is MarketAndKey. Obtain a reference.
    ///
    fn market_and_key_ref<'a, B, Q>(
        decoded_market: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>;
}
