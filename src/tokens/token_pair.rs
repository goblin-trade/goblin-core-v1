use crate::{
    tokens::{DynamicTokenIndex, HardcodedIndex},
    types::Pair,
};

// Token variants
pub struct Hardcoded;
pub struct Dynamic;

pub trait TokenVariant {
    type TokenIndex;
}

impl TokenVariant for Hardcoded {
    type TokenIndex = HardcodedIndex;
}

impl TokenVariant for Dynamic {
    type TokenIndex = DynamicTokenIndex;
}

// Pair shapes
pub trait TokenPairShape<Idx: TokenVariant> {
    type IndexPair;
}

pub struct ETH;
pub struct ERC20;

// ETH–ERC20 and ERC20–ETH => one index
impl<Idx: TokenVariant> TokenPairShape<Idx> for Pair<ETH, ERC20> {
    type IndexPair = Idx::TokenIndex;
}
impl<Idx: TokenVariant> TokenPairShape<Idx> for Pair<ERC20, ETH> {
    type IndexPair = Idx::TokenIndex;
}

// ERC20–ERC20 => pair of indices
impl<Idx: TokenVariant> TokenPairShape<Idx> for Pair<ERC20, ERC20> {
    type IndexPair = Pair<Idx::TokenIndex, Idx::TokenIndex>;
}
