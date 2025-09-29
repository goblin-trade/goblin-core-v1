use crate::{quantities::UnsidedAtoms, tokens::TokenIndex};

/// Generic struct for ERC20 deposits or withdrawals
#[repr(C, packed)]
pub struct ERC20Input<S> {
    pub index: TokenIndex,
    pub amount: UnsidedAtoms,
    _marker: core::marker::PhantomData<S>,
}

/// ERC20 tokens to be deposited
pub struct ERC20Deposit;

/// ERC20 tokens to be withdrawn
pub struct ERC20Withdraw;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TransferDirection {
    Deposit = 0,
    Withdraw = 1,
}

impl Default for TransferDirection {
    fn default() -> Self {
        TransferDirection::Deposit
    }
}

pub trait ERC20Transfer {
    const DIRECTION: TransferDirection;
}

impl ERC20Transfer for ERC20Deposit {
    const DIRECTION: TransferDirection = TransferDirection::Deposit;
}

impl ERC20Transfer for ERC20Withdraw {
    const DIRECTION: TransferDirection = TransferDirection::Withdraw;
}
