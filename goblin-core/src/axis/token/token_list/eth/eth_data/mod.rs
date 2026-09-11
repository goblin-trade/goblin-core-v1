use crate::{
    axis::token::{ETH, ETHStub, token_marker::TokenData},
    settlement::ConstDefault,
};
use core::ops::Index;

pub static ETH_TOKEN_DATA: TokenData<ETH> = TokenData::<ETH>::DEFAULT;

const impl Index<ETHStub> for &TokenData<ETH> {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData::<ETH>::DEFAULT
    }
}
