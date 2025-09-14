use crate::{
    goblin_error::GoblinError,
    quantities::Atoms,
    settlement::{CommonDelta, ERC20Deposit, ERC20Input, ERC20Withdraw},
    tokens::TokenIndex,
    utils::FixedMap,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 16;

/// An expandable list of ERC20 deltas.
pub type ERC20DeltaList = FixedMap<TokenIndex, ERC20Delta, MAX_DELTAS>;

impl ERC20DeltaList {
    /// Create a new ERC20DeltaList initialized with inputs read from args
    pub fn new(
        erc20_deposits_due: &[ERC20Input<ERC20Deposit>],
        erc20_withdrawals_due: &[ERC20Input<ERC20Withdraw>],
    ) -> Result<Self, GoblinError> {
        let mut delta_list = ERC20DeltaList::default();

        for deposit_due in erc20_deposits_due {
            delta_list
                .insert(
                    deposit_due.index,
                    ERC20Delta {
                        deposit_due: deposit_due.amount,
                        withdrawal_due: Atoms::ZERO,
                        common_delta: CommonDelta::default(),
                    },
                )
                .ok_or(GoblinError::ERC20DeltaListFull)?;
        }

        for withdrawal_due in erc20_withdrawals_due {
            delta_list
                .insert(
                    withdrawal_due.index,
                    ERC20Delta {
                        deposit_due: Atoms::ZERO,
                        withdrawal_due: withdrawal_due.amount,
                        common_delta: CommonDelta::default(),
                    },
                )
                .ok_or(GoblinError::ERC20DeltaListFull)?;
        }

        Ok(delta_list)
    }
}
