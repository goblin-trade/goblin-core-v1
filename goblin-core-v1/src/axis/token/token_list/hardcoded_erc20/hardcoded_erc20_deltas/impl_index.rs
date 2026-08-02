use core::ops::{Index, IndexMut};

use crate::{
    axis::token::{
        token_list::hardcoded_erc20::HardcodedERC20Deltas, token_marker::HardcodedERC20Index,
        HardcodedERC20,
    },
    settlement::global_delta::TokenDelta,
};

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
