use crate::axis::token::{token_deltas::TokenDeltas, token_index::TokenIndex, ETHStub, ETH};

#[derive(Clone, Copy)]
pub struct TokenData<T: TokenDeltas> {
    pub address: <T::TokenIndex as TokenIndex>::TokenAddress,
    pub decimals: <T::TokenIndex as TokenIndex>::HardcodedDecimals,
}

impl TokenData<ETH> {
    pub const ETH_STUB_PAIR: (<ETH as TokenDeltas>::TokenIndex, Self) = (
        ETHStub,
        TokenData {
            address: ETHStub,
            decimals: ETHStub,
        },
    );
}
