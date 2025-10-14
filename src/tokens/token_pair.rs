use core::marker::PhantomData;

use crate::{
    markets::HardcodedMarket,
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
