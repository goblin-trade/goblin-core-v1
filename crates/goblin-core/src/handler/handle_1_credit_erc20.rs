use crate::{quantities::Lots, types::Address};

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

    // transfer_from(&params.recipient, &params.recipient, &[0u8; 32]);

    0
}
