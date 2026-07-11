use crate::{
    axis::token::token_index::{HardcodedERC20Index, TokenData},
    goblin_error::GoblinError,
};

pub struct HardcodedTokens<const N: usize> {
    pub inner: [TokenData<HardcodedERC20Index>; N],
}

impl<const N: usize> HardcodedTokens<N> {
    pub fn get(
        &self,
        index: HardcodedERC20Index,
    ) -> Result<&TokenData<HardcodedERC20Index>, GoblinError> {
        self.inner
            .get(index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)
    }

    pub fn typed_iter(
        &self,
    ) -> impl Iterator<Item = (HardcodedERC20Index, TokenData<HardcodedERC20Index>)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .map(|(index, data)| (HardcodedERC20Index::from(index), *data))
    }
}
