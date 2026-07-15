use crate::axis::token::{
    token_marker::TokenData,
    token_marker::{HardcodedERC20Index, HardcodedTokens},
    HardcodedERC20,
};
use core::ops::Index;

impl<const N: usize> Index<HardcodedERC20Index> for HardcodedTokens<N> {
    type Output = TokenData<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}
