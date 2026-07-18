use crate::axis::token::{
    token_marker::{HardcodedERC20List, TokenData},
    HardcodedERC20,
};

impl<'a, const N: usize> IntoIterator for &'a HardcodedERC20List<N> {
    type Item = &'a TokenData<HardcodedERC20>;
    type IntoIter = core::slice::Iter<'a, TokenData<HardcodedERC20>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
