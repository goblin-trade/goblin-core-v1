use crate::{
    axis::token::{
        token_marker::{HardcodedERC20Deltas, HARDCODED_ERC20_COUNT},
        HardcodedERC20,
    },
    settlement::global_delta::TokenDelta,
};

impl IntoIterator for HardcodedERC20Deltas {
    type Item = TokenDelta<HardcodedERC20>;
    type IntoIter = core::array::IntoIter<TokenDelta<HardcodedERC20>, HARDCODED_ERC20_COUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
