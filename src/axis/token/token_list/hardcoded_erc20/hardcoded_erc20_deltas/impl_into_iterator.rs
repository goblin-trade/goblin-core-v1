use crate::{
    axis::token::{token_list::hardcoded_erc20::HardcodedERC20Deltas, HardcodedERC20},
    settlement::global_delta::TokenDelta,
};

impl<'a> IntoIterator for &'a HardcodedERC20Deltas {
    type Item = &'a TokenDelta<HardcodedERC20>;
    type IntoIter = core::slice::Iter<'a, TokenDelta<HardcodedERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
