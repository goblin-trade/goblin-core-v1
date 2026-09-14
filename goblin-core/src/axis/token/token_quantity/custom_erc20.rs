use crate::{
    axis::token::{
        CustomERC20, CustomERC20Stub, token_marker::CustomERC20Index, token_quantity::TokenQuantity,
    },
    quantities::{UnsidedAtoms, UnsidedDeltaLots},
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
    type GlobalDeposit = UnsidedAtoms<i64>;

    type TokenMsgTransfer = CustomERC20Stub;
}
