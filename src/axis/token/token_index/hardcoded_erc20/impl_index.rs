use crate::axis::token::{
    token_index::{HardcodedERC20Index, HardcodedTokens, TokenData},
    HardcodedERC20,
};
use core::ops::{Index, IndexMut};

impl<const N: usize> Index<HardcodedERC20Index> for HardcodedTokens<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl<const N: usize> IndexMut<HardcodedERC20Index> for HardcodedTokens<N> {
    fn index_mut(&mut self, index: HardcodedERC20Index) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}
