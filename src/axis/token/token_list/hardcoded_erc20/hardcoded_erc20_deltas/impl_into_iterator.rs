use crate::{
    axis::token::{
        token_list::hardcoded_erc20::{HardcodedERC20Deltas, HARDCODED_ERC20_COUNT},
        HardcodedERC20,
    },
    settlement::global_delta::TokenDelta,
};

impl IntoIterator for HardcodedERC20Deltas {
    type Item = TokenDelta<HardcodedERC20>;
    type IntoIter = core::array::IntoIter<TokenDelta<HardcodedERC20>, HARDCODED_ERC20_COUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a HardcodedERC20Deltas {
    type Item = &'a TokenDelta<HardcodedERC20>;
    type IntoIter = core::slice::Iter<'a, TokenDelta<HardcodedERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
