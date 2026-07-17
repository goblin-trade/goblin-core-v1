use crate::axis::token::{
    token_marker::{CustomERC20List, TokenData},
    CustomERC20,
};

impl<'a> IntoIterator for CustomERC20List<'a> {
    type Item = TokenData<CustomERC20>;
    type IntoIter = core::iter::Copied<core::slice::Iter<'a, TokenData<CustomERC20>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().copied()
    }
}
