use crate::{
    axis::token::{ETHStub, ETH},
    settlement::{global_delta::TokenDelta, ConstZero},
};
use core::ops::{Index, IndexMut};

pub struct ETHDelta {
    pub inner: TokenDelta<ETH>,
}

impl ConstZero for ETHDelta {
    const ZEROED: Self = Self {
        inner: TokenDelta::ZEROED,
    };
}

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
