use crate::{
    goblin_error::GoblinError,
    types::{Address, Ask, Bid, SideMarker},
    utils::FixedMap,
};

pub const MAX_MAKERS: usize = 16;
pub type PendingMakerUpdates = FixedMap<Address, MakerUpdate, MAX_MAKERS>;

#[derive(Default)]
pub struct MakerUpdate {
    pub bid: MakerUpdateSide<Bid>,
    pub ask: MakerUpdateSide<Ask>,
}

impl MakerUpdate {
    /// Accumulate the matched lots for a given maker on a given side.
    ///
    /// This does not immediately update balances. Instead, it records the
    /// pending effect of a match so that all changes can be applied together
    /// in the settlement phase.
    ///
    /// # Arguments
    /// - `lots`: Lots gained by the maker (credited).
    /// - `lots_opposite`: Lots lost by the maker on the opposite side (debited).
    ///
    /// # Overflow
    /// - `free_lots_in` (credited lots) may overflow; this only affects
    ///   the maker’s credited balance and does not harm solvency.
    /// - `locked_lots_out` (debited lots) must not overflow, but is safe
    ///   since resting orders are always backed by reserves.
    pub fn accumulate_match_result<S: SideMarker>(
        &mut self,
        lots: S::Lots,
        lots_opposite: <S::Opposite as SideMarker>::Lots,
    ) -> Result<(), GoblinError> {
        let deltas_for_side = S::maker_update_for_side_mut(self);

        deltas_for_side.free_lots_in += lots;
        deltas_for_side.locked_lots_out += lots_opposite;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct MakerUpdateSide<S: SideMarker> {
    pub locked_lots_out: <S::Opposite as SideMarker>::Lots,
    pub free_lots_in: S::Lots,
}

impl<S: SideMarker> Default for MakerUpdateSide<S> {
    fn default() -> Self {
        Self {
            locked_lots_out: <S::Opposite as SideMarker>::Lots::default(),
            free_lots_in: S::Lots::default(),
        }
    }
}
