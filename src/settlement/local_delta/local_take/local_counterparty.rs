use crate::{
    axis::{
        leg::{Pair, SamePair},
        update::{SameUpdatePair, UpdatePair},
    },
    quantities::UnsidedLots,
    settlement::ConstZero,
};

/// Pending counterparty update when matched for a side.
///
/// # Convention
///
/// U: UpdateMarker is from perspective of sender.
///
/// * Increase sender: subtract from counterparty locked
/// * Decrease sender: addd to counterparty free
///
/// Since increase and decrease affects different state variables, we cannot use
/// an i64 delta for netting
pub type LocalCounterparty = SameUpdatePair<UnsidedLots>;

impl ConstZero for LocalCounterparty {
    const ZEROED: Self = UpdatePair::new(UnsidedLots::ZEROED, UnsidedLots::ZEROED);
}

impl ConstZero for SamePair<LocalCounterparty> {
    const ZEROED: Self = Pair::new(LocalCounterparty::ZEROED, LocalCounterparty::ZEROED);
}
