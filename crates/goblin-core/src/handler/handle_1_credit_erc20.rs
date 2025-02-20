use core::mem::MaybeUninit;

use crate::{
    erc20::transfer_from,
    log_i64, log_txt, msg_sender,
    quantities::{Atoms, Lots},
    types::Address,
};

pub const HANDLE_1_CREDIT_ERC20: u8 = 1;

#[repr(C, packed)]
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
        let msg = "Looping in handler";
        log_txt(msg.as_ptr(), msg.len());

        for i in 0..20 {
            let byte = params.token[i];
            log_i64(byte as i64);
        }
    }

    // problem here- params.token is corrupted after msg_sender hostio
    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
    }
    // let sender = unsafe {
    //     msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
    //     sender_maybe.assume_init_ref()
    // };

    // This gives
    // [30, 174, 125, 70, 216, 143, 8, 252, 47, 142, 210, 127, 203, 42, 177, 131, 235, 45, 14, 239]
    //
    // The sender address equals to bytes
    // [63, 30, 174, 125, 70, 216, 143, 8, 252, 47, 142, 210, 127, 203, 42, 177, 131, 235, 45, 14]
    //
    // Address bytes are replacing the bytes used in payload!
    unsafe {
        let msg = "Looping after reading sender";
        log_txt(msg.as_ptr(), msg.len());

        for i in 0..20 {
            let byte = params.token[i];
            log_i64(byte as i64);
        }
    }

    0
}
