use crate::{
    axis::token::{token_marker::ETHDelta, ETH},
    settlement::global_delta::TokenDelta,
};

impl IntoIterator for ETHDelta {
    type Item = TokenDelta<ETH>;
    type IntoIter = core::iter::Once<TokenDelta<ETH>>;

    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(self.inner)
    }
}

impl<'a> IntoIterator for &'a ETHDelta {
    type Item = &'a TokenDelta<ETH>;
    type IntoIter = core::iter::Once<&'a TokenDelta<ETH>>;

    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(&self.inner)
    }
}
