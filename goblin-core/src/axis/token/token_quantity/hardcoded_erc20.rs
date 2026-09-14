use crate::{
    axis::token::{
        HardcodedERC20, HardcodedERC20Stub, token_marker::HardcodedERC20Index,
        token_quantity::TokenQuantity,
    },
    quantities::{UnsidedAtoms, UnsidedLots},
    types::Address,
};

impl TokenQuantity for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;
    type TokenAddress = Address;

    type HardcodedDecimals = u8;
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = UnsidedLots<i64>;
    type GlobalDeposit = UnsidedAtoms<i64>;

    type TokenMsgTransfer = HardcodedERC20Stub;
}
