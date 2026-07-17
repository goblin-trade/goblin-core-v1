use core::ops::{Index, IndexMut};

use crate::{
    axis::token::{token_marker::ETHDelta, ETHStub, ETH},
    settlement::global_delta::TokenDelta,
};

impl Index<ETHStub> for ETHDelta {
    type Output = TokenDelta<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &self.inner
    }
}

impl IndexMut<ETHStub> for ETHDelta {
    fn index_mut(&mut self, _index: ETHStub) -> &mut Self::Output {
        &mut self.inner
    }
}
