use crate::axis::token::{token_marker::TokenData, HardcodedERC20};

pub struct HardcodedTokens<const N: usize> {
    pub inner: [TokenData<HardcodedERC20>; N],
}

impl<'a, const N: usize> IntoIterator for &'a HardcodedTokens<N> {
    type Item = TokenData<HardcodedERC20>;
    type IntoIter = core::iter::Copied<core::slice::Iter<'a, TokenData<HardcodedERC20>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().copied()
    }
}
