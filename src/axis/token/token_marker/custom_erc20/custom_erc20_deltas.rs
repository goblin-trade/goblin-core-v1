use crate::{
    axis::token::{
        token_marker::{CustomERC20Index, MAX_CUSTOM_ERC20_COUNT},
        CustomERC20,
    },
    settlement::{global_delta::TokenDelta, ConstZero},
};
use core::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct CustomERC20Deltas {
    pub inner: [TokenDelta<CustomERC20>; MAX_CUSTOM_ERC20_COUNT],
}

impl ConstZero for CustomERC20Deltas {
    const ZEROED: Self = Self {
        inner: [TokenDelta::ZEROED; MAX_CUSTOM_ERC20_COUNT],
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
