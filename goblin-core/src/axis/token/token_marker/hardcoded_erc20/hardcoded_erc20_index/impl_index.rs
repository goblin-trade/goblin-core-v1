use crate::axis::token::{
    HardcodedERC20,
    token_list::hardcoded_erc20::HardcodedERC20List,
    token_marker::{HardcodedERC20Index, TokenData},
};
use core::ops::Index;

const impl<const N: usize> Index<HardcodedERC20Index> for HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.inner]
    }
}

const impl<const N: usize> Index<HardcodedERC20Index> for &HardcodedERC20List<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.inner]
    }
}
