use crate::{
    markets::PairShape,
    quantities::DeltaAtoms,
    token::{ERC20, ETH},
    types::Tuple,
};

// TODO replace with triple

/// Store of deposit amounts for the 3 PairShape variants
pub struct LocalDepositStore {
    pub eth_erc20: <(ETH, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_eth: <(ERC20, ETH) as PairShape>::ResolvedPair<DeltaAtoms>,
    pub erc20_erc20: <(ERC20, ERC20) as PairShape>::ResolvedPair<DeltaAtoms>,
}

impl LocalDepositStore {
    pub const fn zero() -> Self {
        Self {
            eth_erc20: DeltaAtoms::ZERO,
            erc20_eth: DeltaAtoms::ZERO,
            erc20_erc20: Tuple::new2(DeltaAtoms::ZERO, DeltaAtoms::ZERO),
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

impl LocalDeposits<(ETH, ERC20)> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <(ETH, ERC20) as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.eth_erc20
    }
}

impl LocalDeposits<(ERC20, ETH)> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <(ERC20, ETH) as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.erc20_eth
    }
}

impl LocalDeposits<(ERC20, ERC20)> for LocalDepositStore {
    fn deposit_mut(&mut self) -> &mut <(ERC20, ERC20) as PairShape>::ResolvedPair<DeltaAtoms> {
        &mut self.erc20_erc20
    }
}
