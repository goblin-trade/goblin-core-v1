use core::mem::MaybeUninit;

use crate::{
    erc20, events, hostio,
    quantities::{Atoms, Lots},
    state::TraderTokenKey,
    types::Address,
};

pub const HANDLE_3_WITHDRAW_ERC20: u8 = 3;
pub const HANDLE_3_PAYLOAD_LEN: usize = core::mem::size_of::<WithdrawERC20Params>();

#[repr(C, packed)]
struct WithdrawERC20Params {
    /// The token to withdraw
    pub token: Address,

    /// Withdraw lots to `recipient`. This allows a wallet to withdraw to another wallet.
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

pub fn handle_3_withdraw_erc20(payload: &[u8]) -> Result<(), ()> {
    let params = unsafe { &*(payload.as_ptr() as *const WithdrawERC20Params) };

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        hostio::msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let lots_withdrawn = events::withdraw(
        &TraderTokenKey {
            trader: *sender,
            token: params.token,
        },
        params.lots,
    );
    let atoms_withdrawn = Atoms::from(lots_withdrawn);

    erc20::transfer(&params.token, &params.recipient, &atoms_withdrawn)?;

    Ok(())
}
