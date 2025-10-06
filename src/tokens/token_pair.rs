use crate::{
    tokens::{DynamicIndex, HardcodedToken, TokenIndex},
    types::Pair,
};

pub trait MarketVariant {}

// TokenIndex<HardcodedToken> doubles up as marker for hardcoded markets
// DynamicIndex doubles up as marker for dynamic markets
impl MarketVariant for TokenIndex<HardcodedToken> {}
impl MarketVariant for DynamicIndex {}

// Pair shapes
pub trait TokenPairShape<M: MarketVariant> {
    type IndexPair;
}

pub struct ETH;
pub struct ERC20;

// ETH–ERC20 and ERC20–ETH => one index
impl<M: MarketVariant> TokenPairShape<M> for Pair<ETH, ERC20> {
    type IndexPair = M;
}
impl<M: MarketVariant> TokenPairShape<M> for Pair<ERC20, ETH> {
    type IndexPair = M;
}

// ERC20–ERC20 => pair of indices
impl<M: MarketVariant> TokenPairShape<M> for Pair<ERC20, ERC20> {
    type IndexPair = Pair<M, M>;
}
