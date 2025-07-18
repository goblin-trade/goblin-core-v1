use crate::types::Address;

/// This is not straightforward. We update deltas in DeltaList using the token index.
/// Hold off for now, build ix_post_order
pub enum TokenPair {
    EthBasePair(EthBasePair),
    EthQuotePair(EthQuotePair),
    ERC20Pair(ERC20Pair),
}

pub struct EthBasePair {
    pub quote_token: Address,
}

pub struct EthQuotePair {
    pub base_token: Address,
}

pub struct ERC20Pair {
    pub base_token: Address,
    pub quote_token: Address,
}
