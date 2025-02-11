use crate::msg_value;

pub const HANDLE_0_CREDIT_ETH: u8 = 0;

/// Credit ETH to a recipient
pub fn handle_0_credit_eth(payload: &[u8]) -> i32 {
    if payload.len() != 20 {
        return 1;
    }
    let recipient = unsafe { &*(payload.as_ptr() as *const [u8; 20]) };

    let mut amount = [0u8; 32];
    unsafe {
        msg_value(amount.as_mut_ptr());
    }

    // Transfer tokens to 'this.address'

    // Credit tokens to user

    0
}
