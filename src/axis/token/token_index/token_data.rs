use crate::axis::token::{token_marker::TokenMarker, ETHStub, ETH};

#[derive(Clone, Copy)]
pub struct TokenData<T: TokenMarker> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}

impl TokenData<ETH> {
    pub const ETH_STUB_PAIR: (<ETH as TokenMarker>::TokenIndex, Self) = (
        ETHStub,
        TokenData {
            address: ETHStub,
            decimals: ETHStub,
        },
    );
}
