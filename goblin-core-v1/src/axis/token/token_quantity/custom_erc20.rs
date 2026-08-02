use crate::{
    axis::token::{
        token_marker::CustomERC20Index, token_quantity::TokenQuantity, CustomERC20, CustomERC20Stub,
    },
    quantities::{UnsidedDeltaAtoms, UnsidedDeltaLots},
    types::Address,
};

impl TokenQuantity for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;
    type TokenAddress = Address;

    type HardcodedDecimals = CustomERC20Stub;
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = CustomERC20Stub;
}
