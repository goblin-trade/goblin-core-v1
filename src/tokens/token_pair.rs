use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::HardcodedMarket,
    require,
    tokens::{DynamicIndex, HardcodedToken, TokenIndex},
    types::Pair,
};

/// We have 2 market variants
///
/// * Hardcoded market- has hardcoded tokens
/// * Dynamic market- has dynamic tokens that can be either dynamic or custom
///
/// Instead of creating new marker types, we apply this trait on
/// TokenIndex<HardcodedToken> and DynamicIndex directly
pub trait MarketVariant {
    const DISCRIMINATOR: u8;
}

// TokenIndex<HardcodedToken> doubles up as marker for hardcoded markets
// DynamicIndex doubles up as marker for dynamic markets
impl MarketVariant for TokenIndex<HardcodedToken> {
    const DISCRIMINATOR: u8 = 0;
}
impl MarketVariant for DynamicIndex {
    const DISCRIMINATOR: u8 = 1;
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

pub trait TokenPairKind {
    type IndexPair;
}

impl<M: MarketVariant> TokenPairKind for TokenPair<M, Pair<ETH, ERC20>> {
    type IndexPair = M;
}

impl<M: MarketVariant> TokenPairKind for TokenPair<M, Pair<ERC20, ETH>> {
    type IndexPair = M;
}

impl<M: MarketVariant> TokenPairKind for TokenPair<M, Pair<ERC20, ERC20>> {
    type IndexPair = Pair<M, M>;
}

pub trait TokenPairDecoder {
    /// Tells the market type during decoding
    const DISCRIMINATOR: u8;
}

impl<M, P> TokenPairDecoder for TokenPair<M, P>
where
    M: MarketVariant,
    P: PairShape,
{
    const DISCRIMINATOR: u8 = M::DISCRIMINATOR | (P::DISCRIMINATOR << 1);
}

pub trait HardcodedDecoder<P: PairShape + 'static>
where
    TokenPair<TokenIndex<HardcodedToken>, P>: TokenPairKind,
{
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<P>];
}

pub trait DynamicTokenPairDecoder: TokenPairKind {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self::IndexPair, GoblinError>;
}

impl DynamicTokenPairDecoder for TokenPair<DynamicIndex, Pair<ETH, ERC20>> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self::IndexPair, GoblinError> {
        let byte_quote = payload.decode::<u8>(offset, len)?;
        DynamicIndex::decode(byte_quote)
    }
}

impl DynamicTokenPairDecoder for TokenPair<DynamicIndex, Pair<ERC20, ETH>> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self::IndexPair, GoblinError> {
        let byte_base = payload.decode::<u8>(offset, len)?;
        DynamicIndex::decode(byte_base)
    }
}

impl DynamicTokenPairDecoder for TokenPair<DynamicIndex, Pair<ERC20, ERC20>> {
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self::IndexPair, GoblinError> {
        let byte_base = payload.decode::<u8>(offset, len)?;
        let byte_quote = payload.decode::<u8>(offset, len)?;

        // Ensure that token indices are different
        require!(byte_base != byte_quote, GoblinError::InvalidTokenPair);

        let base_index = DynamicIndex::decode(byte_base)?;
        let quote_index = DynamicIndex::decode(byte_quote)?;

        Ok(Pair {
            base: base_index,
            quote: quote_index,
        })
    }
}
