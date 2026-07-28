use crate::{axis::update::SameUpdatePair, quantities::UnsidedLots};

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
