use crate::{
    axis::token::{token_marker::TokenData, ETHStub, ETH},
    settlement::ConstDefault,
};
use core::ops::Index;

pub static ETH_TOKEN_DATA: TokenData<ETH> = TokenData::<ETH>::DEFAULT;

impl<'a> Index<ETHStub> for &'a TokenData<ETH> {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData::<ETH>::DEFAULT
    }
}
