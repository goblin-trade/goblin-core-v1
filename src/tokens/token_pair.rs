use crate::{
    tokens::{DynamicIndex, HardcodedToken, TokenIndex},
    types::Pair,
};

// Market variants
// Instead of Marker marker that maps to token index, we could
// implement MarketVariant on TokenIndex<Hardcoded> and DynamicIndex directly

// pub trait MarketVariantV2 {}
// impl MarketVariantV2 for TokenIndex<HardcodedToken> {}
// impl MarketVariantV2 for DynamicIndex {}

pub struct HardcodedMarker;
pub struct DynamicMarker;

pub trait MarketVariant {
    type TokenIndex;
}

impl MarketVariant for HardcodedMarker {
    type TokenIndex = HardcodedIndex;
}

impl MarketVariant for DynamicMarker {
    type TokenIndex = DynamicIndex;
}

// Pair shapes
pub trait TokenPairShape<M: MarketVariant> {
    type IndexPair;
}

pub struct ETH;
pub struct ERC20;

// ETH–ERC20 and ERC20–ETH => one index
impl<M: MarketVariant> TokenPairShape<M> for Pair<ETH, ERC20> {
    type IndexPair = M::TokenIndex;
}
impl<M: MarketVariant> TokenPairShape<M> for Pair<ERC20, ETH> {
    type IndexPair = M::TokenIndex;
}

// ERC20–ERC20 => pair of indices
impl<M: MarketVariant> TokenPairShape<M> for Pair<ERC20, ERC20> {
    type IndexPair = Pair<M::TokenIndex, M::TokenIndex>;
}
