use crate::axis::token::{
    token_list::hardcoded_erc20::HardcodedERC20List,
    token_marker::{HardcodedERC20Index, TokenData},
    HardcodedERC20,
};
use core::ops::Index;

impl<const N: usize> Index<HardcodedERC20Index> for HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.inner]
    }
}

impl<const N: usize> Index<HardcodedERC20Index> for &HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.inner]
    }
}
