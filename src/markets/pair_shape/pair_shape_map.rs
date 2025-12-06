use crate::{
    markets::{PairShape, ERC20, ETH},
    types::Pair,
};

/// Stores one value of `T` for each supported token-pair shape.
pub struct PairShapeMap<T> {
    pub eth_erc20: T,
    pub erc20_eth: T,
    pub erc20_erc20: T,
}

pub trait PairShapeGetter<T, P: PairShape> {
    fn get_ref(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl<T> PairShapeGetter<T, Pair<ETH, ERC20>> for PairShapeMap<T> {
    fn get_ref(&self) -> &T {
        &self.eth_erc20
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.eth_erc20
    }
}

impl<T> PairShapeGetter<T, Pair<ERC20, ETH>> for PairShapeMap<T> {
    fn get_ref(&self) -> &T {
        &self.erc20_eth
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.erc20_eth
    }
}

impl<T> PairShapeGetter<T, Pair<ERC20, ERC20>> for PairShapeMap<T> {
    fn get_ref(&self) -> &T {
        &self.erc20_erc20
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.erc20_erc20
    }
}
