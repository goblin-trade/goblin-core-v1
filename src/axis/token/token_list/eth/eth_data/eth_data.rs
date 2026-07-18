use crate::{
    axis::token::{token_marker::TokenData, ETHStub, ETH},
    settlement::ConstZero,
};
use core::ops::Index;

pub static ETH_TOKEN_DATA: TokenData<ETH> = TokenData::<ETH>::ZEROED;

impl ConstZero for TokenData<ETH> {
    const ZEROED: Self = TokenData {
        address: ETHStub,
        decimals: ETHStub,
    };
}

impl<'a> Index<ETHStub> for &'a TokenData<ETH> {
    type Output = TokenData<ETH>;

    fn index(&self, _index: ETHStub) -> &Self::Output {
        &TokenData::<ETH>::ZEROED
    }
}
