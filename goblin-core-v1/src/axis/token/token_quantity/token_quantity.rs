use crate::{
    axis::token::token_msg_transfer::TokenMsgTransfer,
    input_processor::Decodable,
    quantities::UnsidedDeltaAtoms,
    settlement::{CheckedOps, ConstZero},
};

pub trait TokenQuantity: Clone + Copy + PartialEq + 'static {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq;

    type TokenAddress: Clone + Copy + Sized + Default;

    /// Decimals hardcoded in the smart contract
    type HardcodedDecimals: Clone + Copy;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy + Into<u8>;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone
        + Copy
        + Default
        + PartialEq
        + Decodable
        + ConstZero
        + CheckedOps
        + Into<UnsidedDeltaAtoms>;

    type TokenMsgTransfer: TokenMsgTransfer;
}
