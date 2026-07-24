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
pub type CounterpartyUpdate = SameUpdatePair<UnsidedLots>;

impl ConstZero for CounterpartyUpdate {
    const ZEROED: Self = UpdatePair::new(UnsidedLots::ZEROED, UnsidedLots::ZEROED);
}

impl ConstZero for SamePair<CounterpartyUpdate> {
    const ZEROED: Self = Pair::new(CounterpartyUpdate::ZEROED, CounterpartyUpdate::ZEROED);
}
