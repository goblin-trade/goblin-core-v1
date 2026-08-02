use crate::axis::token::{
    token_list::custom_erc20::CustomERC20List, token_marker::TokenData, CustomERC20,
};

impl<'a> IntoIterator for CustomERC20List<'a> {
    type Item = &'a TokenData<CustomERC20>;
    type IntoIter = core::slice::Iter<'a, TokenData<CustomERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
