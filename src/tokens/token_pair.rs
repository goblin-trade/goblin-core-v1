use core::marker::PhantomData;

use crate::{
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
    const DECODE_FLAG: u8;
}

// TokenIndex<HardcodedToken> doubles up as marker for hardcoded markets
// DynamicIndex doubles up as marker for dynamic markets
impl MarketVariant for TokenIndex<HardcodedToken> {
    const DECODE_FLAG: u8 = 0;
}
impl MarketVariant for DynamicIndex {
    const DECODE_FLAG: u8 = 1;
}

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait PairShape {
    const DECODE_FLAG: u8;
}

impl PairShape for Pair<ETH, ERC20> {
    const DECODE_FLAG: u8 = 0;
}

impl PairShape for Pair<ERC20, ETH> {
    const DECODE_FLAG: u8 = 1;
}

impl PairShape for Pair<ERC20, ERC20> {
    const DECODE_FLAG: u8 = 2;
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
