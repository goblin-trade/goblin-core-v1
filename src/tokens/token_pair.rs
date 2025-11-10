use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::HardcodedMarket,
    require,
    state::{DynamicMarketKey, HardcodedMarketKey, SlotKey},
    tokens::{DynamicIndex, HardcodedIndex},
    types::Pair,
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
pub trait MarketVariant {
    const DISCRIMINATOR: u8;

    /// Key to read market slot
    type MarketKey<P: PairShape>: SlotKey;
}

impl MarketVariant for HardcodedIndex {
    const DISCRIMINATOR: u8 = 0;

    type MarketKey<P: PairShape> = HardcodedMarketKey<P>;
}

impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;

    type MarketKey<P: PairShape> = DynamicMarketKey<P>;
}

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait PairShape {
    const DISCRIMINATOR: u8;
}

impl PairShape for Pair<ETH, ERC20> {
    const DISCRIMINATOR: u8 = 0;
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;
}

/// A pair of two ERC20 tokens.
/// The decoder guarantees that the two tokens are different.
impl PairShape for Pair<ERC20, ERC20> {
    const DISCRIMINATOR: u8 = 2;
}

/// Combined pair with 2 traits
pub struct TokenPair<M: MarketVariant, P: PairShape>(PhantomData<(M, P)>);

/// Maps marker structs to token indices
pub trait IndexPairFor {
    type ResolvedIndexPair;
}

impl<M: MarketVariant> IndexPairFor for TokenPair<M, Pair<ETH, ERC20>> {
    type ResolvedIndexPair = M;
}

impl<M: MarketVariant> IndexPairFor for TokenPair<M, Pair<ERC20, ETH>> {
    type ResolvedIndexPair = M;
}

impl<M: MarketVariant> IndexPairFor for TokenPair<M, Pair<ERC20, ERC20>> {
    type ResolvedIndexPair = Pair<M, M>;
}

/// Map each PairShape to a hardcoded market list
pub trait HardcodedMarketList<P: PairShape + 'static>
where
    TokenPair<HardcodedIndex, P>: IndexPairFor,
{
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<P>];
}

// -----------------------------------
// Impls to decode dynamic token pairs

impl Decodable<<Self as IndexPairFor>::ResolvedIndexPair>
    for TokenPair<DynamicIndex, Pair<ETH, ERC20>>
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Self as IndexPairFor>::ResolvedIndexPair, GoblinError> {
        let byte_quote = payload.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Self as IndexPairFor>::ResolvedIndexPair>
    for TokenPair<DynamicIndex, Pair<ERC20, ETH>>
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Self as IndexPairFor>::ResolvedIndexPair, GoblinError> {
        let byte_quote = payload.decode::<u8>(offset, len)?;
        DynamicIndex::new(byte_quote)
    }
}

impl Decodable<<Self as IndexPairFor>::ResolvedIndexPair>
    for TokenPair<DynamicIndex, Pair<ERC20, ERC20>>
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<<Self as IndexPairFor>::ResolvedIndexPair, GoblinError> {
        let byte_base = payload.decode::<u8>(offset, len)?;
        let byte_quote = payload.decode::<u8>(offset, len)?;

        require!(byte_base != byte_quote, GoblinError::InvalidTokenPair);

        let base_index = DynamicIndex::new(byte_base)?;
        let quote_index = DynamicIndex::new(byte_quote)?;

        Ok(Pair {
            base: base_index,
            quote: quote_index,
        })
    }
}
