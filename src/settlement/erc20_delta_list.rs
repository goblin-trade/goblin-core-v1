use crate::{
    goblin_error::GoblinError,
    settlement::{ERC20Deposit, ERC20Input, ERC20Withdraw},
    tokens::DynamicIndex,
    utils::FixedMap,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_DELTAS: usize = 16;

/// An expandable list of ERC20 deltas.
///
/// # Design decision
///
/// Use a common list of DynamicIndex tokens, instead of separate lists for HardcodedIndex
/// and CustomIndex. Using two lists is not memory efficient as the lists will be sparse.
///
/// # What if we use 2 lists
/// - This will take T: ERC20TokenTrait as generic
/// - (8, 8) distribution can be too less? Majority tokens- ETH, USDC, USDT, BTC, Arb. Well I don't think so.
/// We could compress to (8, 4). Max 4 custom tokens.
/// - ERC20Input however uses dynamic index. It is not
pub type ERC20DeltaList = FixedMap<DynamicIndex, ERC20Delta, MAX_DELTAS>;

// impl ERC20DeltaList {
//     /// Create a new ERC20DeltaList initialized with inputs read from args
//     ///
//     /// UNUSED FUNCTION
//     /// * I am yet to write a new implementation that uses Pair generic.
//     /// * Deposits and withdrawals are now namespaced by market pair type and shape. Eg. ETH cannot be deposited.
//     pub fn new(
//         erc20_deposits_due: &[ERC20Input<ERC20Deposit>],
//         erc20_withdrawals_due: &[ERC20Input<ERC20Withdraw>],
//     ) -> Result<Self, GoblinError> {
//         let mut delta_list = ERC20DeltaList::default();

//         for deposit_due in erc20_deposits_due {
//             delta_list
//                 .insert(deposit_due.index, ERC20Delta::new(deposit_due))
//                 .ok_or(GoblinError::ERC20DeltaListFull)?;
//         }

//         for withdrawal_due in erc20_withdrawals_due {
//             delta_list
//                 .insert(withdrawal_due.index, ERC20Delta::new(withdrawal_due))
//                 .ok_or(GoblinError::ERC20DeltaListFull)?;
//         }

//         Ok(delta_list)
//     }
// }
