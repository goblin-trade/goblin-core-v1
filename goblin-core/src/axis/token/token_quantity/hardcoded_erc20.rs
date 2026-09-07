use crate::{
    axis::token::{
        token_marker::HardcodedERC20Index, token_quantity::TokenQuantity, HardcodedERC20,
        HardcodedERC20Stub,
    },
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaLots},
    types::Address,
};

impl TokenQuantity for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;
    type TokenAddress = Address;

    type HardcodedDecimals = u8;
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = HardcodedERC20Stub;
}
