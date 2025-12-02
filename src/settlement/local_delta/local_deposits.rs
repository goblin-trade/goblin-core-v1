use crate::{
    markets::{PairShape, ERC20, ETH},
    quantities::DeltaAtoms,
    types::Pair,
};

pub struct LocalDeposits {
    pub eth_erc20: <Pair<ETH, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_eth: <Pair<ERC20, ETH> as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_erc20: <Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>,
}

impl LocalDeposits {
    pub const fn new() -> Self {
        Self {
            eth_erc20: DeltaAtoms::ZERO,
            erc20_eth: DeltaAtoms::ZERO,
            erc20_erc20: Pair {
                base: DeltaAtoms::ZERO,
                quote: DeltaAtoms::ZERO,
            },
        }
    }
}
