use crate::{
    axis::update::{SameUpdatePair, UpdatePair},
    quantities::UnsidedAtoms,
    settlement::ConstZero,
};

/// Pending counterparty update for a token
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
pub type GlobalCounterparty = SameUpdatePair<UnsidedAtoms>;

impl ConstZero for GlobalCounterparty {
    const ZEROED: Self = UpdatePair::new(UnsidedAtoms::ZEROED, UnsidedAtoms::ZEROED);
}
