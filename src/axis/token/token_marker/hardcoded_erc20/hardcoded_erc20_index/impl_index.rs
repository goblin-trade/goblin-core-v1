use crate::axis::token::{
    token_marker::TokenData,
    token_marker::{HardcodedERC20Index, HardcodedERC20List},
    HardcodedERC20,
};
use core::ops::Index;

impl<const N: usize> Index<HardcodedERC20Index> for HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl<const N: usize> Index<HardcodedERC20Index> for &HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}
