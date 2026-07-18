use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20Deltas, CustomERC20},
    settlement::global_delta::TokenDelta,
};

impl<'a> IntoIterator for &'a CustomERC20Deltas {
    type Item = &'a TokenDelta<CustomERC20>;
    type IntoIter = core::slice::Iter<'a, TokenDelta<CustomERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
