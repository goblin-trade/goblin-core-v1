use crate::{axis::update::SameUpdatePair, quantities::UnsidedAtoms};

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
