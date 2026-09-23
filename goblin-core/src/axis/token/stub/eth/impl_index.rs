use core::ops::Index;

use crate::{
    axis::token::{ETH, token_marker::TokenData},
    settlement::ConstDefault,
};

use super::ETHStub;

impl Index<ETHStub> for ETHStub {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData::DEFAULT
    }
}
