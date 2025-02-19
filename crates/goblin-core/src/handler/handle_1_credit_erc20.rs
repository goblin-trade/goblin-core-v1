use core::mem::MaybeUninit;

use crate::{
    erc20::transfer_from,
    log_i64, log_txt, msg_sender,
    quantities::{Atoms, Lots},
    types::Address,
};

pub const HANDLE_1_CREDIT_ERC20: u8 = 1;

#[repr(C)]
struct CreditERC20Params {
    /// The token to credit
    pub token: Address,

    /// The recipient of the funds. Funds can be credited to any address
    pub recipient: Address,

    /// The lots to credit. Atom to lot conversions should happen on client side.
    ///
    /// The lots bytes should be encoded in **little endian** for zero copy deserialization.
    ///
    /// For 1 lot
    /// - Correct (little endian, non ABI): 0x0100000000000000 = [0x01, 0x00, ...]
    /// - Wrong (big endian, ABI style): 0x0000000000000001 = [0x00, 0x00, ..., 0x01]
    pub lots: Lots,
}

/// Credit an ERC20 token to a recipient
pub fn handle_1_credit_erc20(payload: &[u8]) -> i32 {
    if payload.len() < core::mem::size_of::<CreditERC20Params>() {
        return 1;
    }

    let params = unsafe { &*(payload.as_ptr() as *const CreditERC20Params) };

    unsafe {
        let msg = "Token byte 0";
        log_txt(msg.as_ptr(), msg.len());

        log_i64(params.token[0] as i64);

        let msg = "Token byte 19";
        log_txt(msg.as_ptr(), msg.len());

        log_i64(params.token[19] as i64);
    }

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    let atoms = Atoms::from(&params.lots);
    let result = transfer_from(&params.token, sender, &params.recipient, &atoms);

    unsafe {
        let msg = b"Call result";
        log_txt(msg.as_ptr(), msg.len());
        log_i64(result as i64);
    }

    if result != 0 {
        return 1;
    }

    // TODO test
    0
}
