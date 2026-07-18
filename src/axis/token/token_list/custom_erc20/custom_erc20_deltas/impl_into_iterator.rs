use crate::{
    axis::token::{
        token_list::custom_erc20::{CustomERC20Deltas, MAX_CUSTOM_ERC20_COUNT},
        CustomERC20,
    },
    settlement::global_delta::TokenDelta,
};

impl IntoIterator for CustomERC20Deltas {
    type Item = TokenDelta<CustomERC20>;
    type IntoIter = core::array::IntoIter<TokenDelta<CustomERC20>, MAX_CUSTOM_ERC20_COUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a CustomERC20Deltas {
    type Item = &'a TokenDelta<CustomERC20>;
    type IntoIter = core::slice::Iter<'a, TokenDelta<CustomERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
