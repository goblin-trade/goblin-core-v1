use crate::{
    axis::token::{token_index::CustomERC20Index, CustomERC20},
    settlement::{global_delta::TokenDelta, ConstZero},
};
use core::ops::{Index, IndexMut};

pub const MAX_CUSTOM_DELTAS: usize = 8;

#[derive(Clone, Copy)]
pub struct CustomERC20Deltas {
    pub inner: [TokenDelta<CustomERC20>; MAX_CUSTOM_DELTAS],
}

impl ConstZero for CustomERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; MAX_CUSTOM_DELTAS],
    };
}

impl Index<CustomERC20Index> for CustomERC20Deltas {
    type Output = TokenDelta<CustomERC20>;

    fn index(&self, index: CustomERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}

impl IndexMut<CustomERC20Index> for CustomERC20Deltas {
    fn index_mut(&mut self, index: CustomERC20Index) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}
