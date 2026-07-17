use crate::axis::token::{token_quantity::TokenQuantity, ETHStub, ETH};

#[derive(Clone, Copy)]
pub struct TokenData<T: TokenQuantity> {
    pub address: T::TokenAddress,
    pub decimals: T::HardcodedDecimals,
}

impl TokenData<ETH> {
    pub const ETH_STUB_PAIR: (<ETH as TokenQuantity>::TokenIndex, Self) = (
        ETHStub,
        TokenData {
            address: ETHStub,
            decimals: ETHStub,
        },
    );
}
