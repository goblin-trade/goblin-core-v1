use core::mem::MaybeUninit;

use crate::{
    eth, events, msg_sender,
    quantities::{Atoms, Lots},
    state::TraderTokenKey,
    types::{Address, NATIVE_TOKEN},
};

pub const HANDLE_2_WITHDRAW_ETH: u8 = 2;
pub const HANDLE_2_PAYLOAD_LEN: usize = core::mem::size_of::<WithdrawETHParams>();

// We need packed otherwise the struct will be of size 32 not 28
// handle_1_credit_erc20 case is an exception. It worked without packed because 2 addresses
// equal to 40 bytes, a multiple of 8
#[repr(C, packed)]
struct WithdrawETHParams {
    /// Withdraw lots to `recipient`. This allows a wallet to withdraw to another wallet
    pub recipient: Address,

    /// The lots to withdraw. Atom to lot conversions should happen on client side.
    ///
    /// If the value is greater than than the deposited amount, entire deposit
    /// is withdrawn.
    ///
    /// The lots bytes should be encoded in **little endian** for zero copy deserialization.
    ///
    /// For 1 lot
    /// - Correct (little endian, non ABI): 0x0100000000000000 = [0x01, 0x00, ...]
    /// - Wrong (big endian, ABI style): 0x0000000000000001 = [0x00, 0x00, ..., 0x01]
    pub lots: Lots,
}

pub fn handle_2_withdraw_eth(payload: &[u8]) -> Result<(), ()> {
    let params = unsafe { &*(payload.as_ptr() as *const WithdrawETHParams) };

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let lots_withdrawn = events::withdraw(
        &TraderTokenKey {
            trader: *sender,
            token: NATIVE_TOKEN,
        },
        params.lots,
    );
    // TODO we must flush cache before cross contract call?
    let atoms_withdrawn = Atoms::from(lots_withdrawn);
    eth::transfer_out(&params.recipient, &atoms_withdrawn)?;

    Ok(())
}
