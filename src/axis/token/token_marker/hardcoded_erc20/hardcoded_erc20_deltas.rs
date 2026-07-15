use crate::{
    axis::token::{
        token_marker::{HardcodedERC20Index, HARDCODED_ERC20_COUNT},
        HardcodedERC20,
    },
    settlement::{global_delta::TokenDelta, ConstZero},
};
use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_ERC20_COUNT],
}

impl ConstZero for HardcodedERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; HARDCODED_ERC20_COUNT],
    };
}

impl Index<HardcodedERC20Index> for HardcodedERC20Deltas {
    type Output = TokenDelta<HardcodedERC20>;

    fn index(&self, index: HardcodedERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl IndexMut<HardcodedERC20Index> for HardcodedERC20Deltas {
    fn index_mut(&mut self, index: HardcodedERC20Index) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}

impl IntoIterator for HardcodedERC20Deltas {
    type Item = TokenDelta<HardcodedERC20>;
    type IntoIter = core::array::IntoIter<TokenDelta<HardcodedERC20>, HARDCODED_ERC20_COUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
