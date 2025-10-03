use crate::{
    tokens::{DynamicTokenIndex, HardcodedIndex},
    types::Pair,
};

/// Generic pair of tokens
///
/// * `TokenPair<HardcodedIndex>` → only hardcoded tokens. It uses a 'struct' type HardcodedIndex.
/// * `TokenPair<TokenIndex>` → dynamic (runtime) tokens, can be hardcoded or custom.
/// It uses an 'enum' type TokenIndex.
pub enum TokenPair<T> {
    ETHBaseERC20Quote(T),
    ERC20BaseETHQuote(T),
    ERC20BaseERC20Quote(Pair<T, T>),
}

// Type aliases for clarity
pub type HardcodedPair = TokenPair<HardcodedIndex>;
pub type DynamicPair = TokenPair<DynamicTokenIndex>;
