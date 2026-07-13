use crate::{
    axis::token::{
        token_index::{HardcodedERC20Index, TokenData},
        HardcodedERC20,
    },
    goblin_error::GoblinError,
};

pub struct HardcodedTokens<const N: usize> {
    pub inner: [TokenData<HardcodedERC20>; N],
}

impl<const N: usize> HardcodedTokens<N> {
    pub fn get(
        &self,
        index: HardcodedERC20Index,
    ) -> Result<&TokenData<HardcodedERC20>, GoblinError> {
        self.inner
            .get(index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)
    }
}
