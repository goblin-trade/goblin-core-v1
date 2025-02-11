pub const HANDLE_1_CREDIT_ERC20: u8 = 1;

struct CreditERC20Params {
    /// The token to credit
    pub token: [u8; 20],

    /// The recipient of the funds. Funds can be credited to any address
    pub recipient: [u8; 20],

    /// The amount to credit
    pub amount: [u8; 32],
}

/// Credit free funds to a recipient
pub fn handle_1_credit_erc20(payload: &[u8]) -> i32 {
    if payload.len() < core::mem::size_of::<CreditERC20Params>() {
        return 1;
    }
    let credit_funds_params = unsafe { &*(payload.as_ptr() as *const CreditERC20Params) };

    // Transfer tokens to 'this.address'

    // Credit tokens to user

    0
}
