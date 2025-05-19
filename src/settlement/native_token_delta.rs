/// Amount of native tokens due to be transferred on settlement
///
/// Native token delta is tracked separately from TokenDeltaList because
/// * There is no address to track
/// * `native_withdrawal_due` can only have positive sign. ETH deposits
/// happen a-priori via msg.value, not during settlement.
pub struct NativeTokenDelta {
    /// atoms due to be deducted from TraderTokenState (slot) on settlement
    ///
    /// * Positive: Deduct from TraderTokenState
    /// * Negative: add to TraderTokenState
    ///
    /// When tokens are used up to place orders, increase the delta. This delta
    /// must be squared off from TraderTokenState. Conversely if delta is negative,
    /// square off by crediting atoms to TraderTokenState
    ///
    /// TraderTokenState should have sufficient balance to cover slot_deduction_due
    /// on settlement, else the TX will revert due to insufficient funds.
    pub slot_deduction_due: i64,

    /// atoms due to be transferred out to trader's ETH balance on settlement
    pub native_withdrawal_due: i64,
}

impl NativeTokenDelta {
    pub fn execute_and_settle_deposit(&mut self, amount: i64) {
        assert!(amount > 0);
        self.slot_deduction_due -= amount;
    }

    pub fn execute_withdraw(&mut self, amount: i64) {
        // Debug statement ensures that function is not used for deposits (amount < 0)
        debug_assert!(amount > 0);

        self.slot_deduction_due += amount;
        self.native_withdrawal_due += amount;
    }

    pub fn settle_withdraw(&mut self) {
        // TODO ensure that TraderTokenState can cover slot_deduction_due
        // TODO transfer out native_withdrawal_due
    }
}
