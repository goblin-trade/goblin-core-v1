mod custom_erc20;
mod eth;
mod hardcoded_erc20;

use super::TokenMsgTransfer;
use crate::{
    quantities::UnsidedAtoms,
    settlement::{CheckedOps, ConstDefault},
};

pub trait TokenQuantity: Clone + Copy + PartialEq + 'static {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone
        + Copy
        + ConstDefault
        + PartialEq
        + From<usize>
        + for<'a> deku::DekuReader<'a>;

    type TokenAddress: Clone + Copy + Sized + Default + ConstDefault;

    /// Decimals hardcoded in the smart contract
    type HardcodedDecimals: Clone + Copy + ConstDefault;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy + Into<u8>;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone
        + Copy
        + Default
        + ConstDefault
        + CheckedOps
        + for<'a> deku::DekuReader<'a>;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone
        + Copy
        + Default
        + PartialEq
        + ConstDefault
        + CheckedOps
        + Into<UnsidedAtoms<i64>>
        + for<'a> deku::DekuReader<'a>;

    type TokenMsgTransfer: TokenMsgTransfer;
}
