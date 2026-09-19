use core::ops::{Index, IndexMut};

use crate::{
    axis::token::{
        CustomERC20, token_list::custom_erc20::CustomERC20Deltas, token_marker::CustomERC20Index,
    },
    settlement::global_delta::TokenDelta,
};

impl Index<CustomERC20Index> for CustomERC20Deltas {
    type Output = TokenDelta<CustomERC20>;

    fn index(&self, index: CustomERC20Index) -> &Self::Output {
        &self.inner[index.inner]
    }
}

impl IndexMut<CustomERC20Index> for CustomERC20Deltas {
    fn index_mut(&mut self, index: CustomERC20Index) -> &mut Self::Output {
        &mut self.inner[index.inner]
    }
}
