use crate::{
    axis::token::{
        token_index::{HardcodedERC20Index, HARDCODED_TOKEN_COUNT},
        HardcodedERC20,
    },
    settlement::{global_delta::TokenDelta, ConstZero},
};
use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct HardcodedERC20Deltas {
    pub inner: [TokenDelta<HardcodedERC20>; HARDCODED_TOKEN_COUNT],
}

impl ConstZero for HardcodedERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; HARDCODED_TOKEN_COUNT],
    };
}

// TODO trait with settle function
// This should settle all elements. It should also include Index and IndexMut traits

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
