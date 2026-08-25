use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        update::{SameUpdatePair, UpdateMarker},
    },
    goblin_error::GoblinError,
    quantities::{UnsideQuantity, UnsidedLots},
    settlement::CheckedOps,
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
pub type LocalCounterparty = SamePair<SameUpdatePair<UnsidedLots>>;

impl LocalCounterparty {
    pub fn add_leg<UM: UpdateMarker, In: LegMatcher>(
        &mut self,
        lots: In::Lots,
    ) -> Result<(), GoblinError> {
        let total_delta = UM::get_leg_mut(In::get_leg_mut(self));

        *total_delta = total_delta
            .checked_add(lots.unsided())
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
