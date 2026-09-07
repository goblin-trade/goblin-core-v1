unsafe extern "C" {
    // Have a single TraderTokenState update event instead
    // The amount deposited or withdrawn can be reconstructed using SQL
    //
    // However we already store slot writes by frame
    // We can reconstruct TraderTokenKey using (trader, token), read slots
    // and reconstruct deposits and withdrawals using SQL
    //
    // Use case- get deposit and withdrawal history of a trader
    pub fn index_deposit(recipient: *const u8, token: *const u8, atoms: u64);
    pub fn index_withdraw(recipient: *const u8, token: *const u8, atoms: u64);
}
