use crate::axis::token::{
    token_marker::{HardcodedERC20List, TokenData},
    HardcodedERC20,
};

impl<'a, const N: usize> IntoIterator for &'a HardcodedERC20List<N> {
    type Item = TokenData<HardcodedERC20>;
    type IntoIter = core::iter::Copied<core::slice::Iter<'a, TokenData<HardcodedERC20>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().copied()
    }
}
