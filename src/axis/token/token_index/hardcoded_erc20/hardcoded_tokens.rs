use crate::{
    axis::token::token_index::{HardcodedERC20Data, HardcodedERC20Index},
    goblin_error::GoblinError,
};

pub struct HardcodedTokens<const N: usize> {
    pub inner: [HardcodedERC20Data; N],
}

impl<const N: usize> HardcodedTokens<N> {
    pub fn get(&self, index: HardcodedERC20Index) -> Result<&HardcodedERC20Data, GoblinError> {
        self.inner
            .get(index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)
    }

    pub fn typed_iter(
        &self,
    ) -> impl Iterator<Item = (HardcodedERC20Index, HardcodedERC20Data)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .map(|(index, data)| (HardcodedERC20Index::from(index), *data))
    }
}
