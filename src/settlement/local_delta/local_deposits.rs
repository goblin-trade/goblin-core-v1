use crate::{
    markets::{PairShape, ERC20, ETH},
    quantities::DeltaAtoms,
    types::Pair,
};

/// Store of deposit amounts for the 3 PairShape variants
pub struct LocalDepositStore {
    pub eth_erc20: <Pair<ETH, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_eth: <Pair<ERC20, ETH> as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_erc20: <Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DeltaAtoms>,
}

impl LocalDepositStore {
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

/// Trait that lets a pair-shape pick the correct deposit field
pub trait LocalDeposits<P>
where
    P: PairShape,
    P::ResolvedPair<DeltaAtoms>: Default,
{
    fn deposit_mut(&mut self) -> &mut P::ResolvedPair<DeltaAtoms>;

    /// Reset deposit amounts for P
    ///
    /// # Optimization
    ///
    /// No need to zero fill the entire struct. Only reset what was used.
    fn reset(&mut self) {
        let deposit_pair = self.deposit_mut();
        *deposit_pair = P::ResolvedPair::<DeltaAtoms>::default();
    }
}

impl LocalDeposits<Pair<ETH, ERC20>> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <Pair<ETH, ERC20> as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.eth_erc20
    }
}

impl LocalDeposits<Pair<ERC20, ETH>> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <Pair<ERC20, ETH> as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.erc20_eth
    }
}

impl LocalDeposits<Pair<ERC20, ERC20>> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <Pair<ERC20, ERC20> as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.erc20_erc20
    }
}
