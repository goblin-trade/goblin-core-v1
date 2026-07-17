use crate::{
    axis::token::{token_quantity::TokenQuantity, ETHStub, ETH},
    input_processor::ETHTransfers,
};

impl TokenQuantity for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ETHStub;
    type TokenAddress = ETHStub;

    type HardcodedDecimals = ETHStub;
    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    type TokenMsgTransfer = ETHTransfers;
}
