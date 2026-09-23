use crate::axis::token::{ETH, token_marker::TokenData};

impl<'a> IntoIterator for &'a TokenData<ETH> {
    type Item = &'a TokenData<ETH>;
    type IntoIter = core::iter::Once<&'a TokenData<ETH>>;

    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(self)
    }
}
