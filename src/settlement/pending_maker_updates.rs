use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    types::{Address, Ask, Bid, SideMarker},
    utils::FixedMap,
};

pub const MAX_MAKERS: usize = 15;

pub type PendingMakerUpdatesV2 = FixedMap<Address, MakerUpdate, MAX_MAKERS>;

pub struct PendingMakerUpdates {
    inner: [MaybeUninit<MakerUpdate>; MAX_MAKERS],
    pub len: usize,
}

impl Default for PendingMakerUpdates {
    fn default() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; MAX_MAKERS],
            len: 0,
        }
    }
}

impl PendingMakerUpdates {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MakerUpdate> {
        self.inner[..self.len]
            .iter_mut()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_mut() })
    }

    pub fn get_or_insert(&mut self, maker: &Address) -> Result<&mut MakerUpdate, GoblinError> {
        // First, try to find existing delta with this token index
        for i in 0..self.len {
            let delta_ref = unsafe { self.inner[i].assume_init_ref() };
            if delta_ref.address == *maker {
                return Ok(unsafe { self.inner[i].assume_init_mut() });
            }
        }

        // Not found, insert new delta
        if self.len >= MAX_MAKERS {
            return Err(GoblinError::DeltaListFull);
        }

        // Insert new delta at the end
        let new_delta = MakerUpdate::new(*maker);
        self.inner[self.len].write(new_delta);
        self.len += 1;

        // Return reference to the newly inserted delta
        Ok(unsafe { self.inner[self.len - 1].assume_init_mut() })
    }

    /// Accumulate the matched lots for a given maker on a given side.
    ///
    /// This does not immediately update balances. Instead, it records the
    /// pending effect of a match so that all changes can be applied together
    /// in the settlement phase.
    ///
    /// # Arguments
    /// - `maker`: Address of the maker whose resting order was matched.
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
        maker: &Address,
        lots: S::Lots,
        lots_opposite: <S::Opposite as SideMarker>::Lots,
    ) -> Result<(), GoblinError> {
        let market_maker_delta = self.get_or_insert(maker)?;
        let deltas_for_side = S::maker_deltas_for_side(market_maker_delta);

        deltas_for_side.free_lots_in += lots;
        deltas_for_side.locked_lots_out += lots_opposite;

        Ok(())
    }
}

pub struct MakerUpdate {
    pub address: Address,

    pub bid: MakerUpdateSide<Bid>,
    pub ask: MakerUpdateSide<Ask>,
}

impl MakerUpdate {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            bid: MakerUpdateSide::<Bid>::default(),
            ask: MakerUpdateSide::<Ask>::default(),
        }
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
