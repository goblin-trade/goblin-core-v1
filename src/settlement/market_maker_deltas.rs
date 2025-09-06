use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    types::{Address, Ask, Bid, SideMarker},
};

pub const MAX_MARKET_MAKERS: usize = 15;

pub struct MarketMakerDeltas {
    inner: [MaybeUninit<MarketMakerDelta>; MAX_MARKET_MAKERS],
    pub len: usize,
}

impl Default for MarketMakerDeltas {
    fn default() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; MAX_MARKET_MAKERS],
            len: 0,
        }
    }
}

impl MarketMakerDeltas {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MarketMakerDelta> {
        self.inner[..self.len]
            .iter_mut()
            .map(|maybe_uninit| unsafe { maybe_uninit.assume_init_mut() })
    }

    pub fn get_or_insert(&mut self, maker: &Address) -> Result<&mut MarketMakerDelta, GoblinError> {
        // First, try to find existing delta with this token index
        for i in 0..self.len {
            let delta_ref = unsafe { self.inner[i].assume_init_ref() };
            if delta_ref.address == *maker {
                return Ok(unsafe { self.inner[i].assume_init_mut() });
            }
        }

        // Not found, insert new delta
        if self.len >= MAX_MARKET_MAKERS {
            return Err(GoblinError::DeltaListFull);
        }

        // Insert new delta at the end
        let new_delta = MarketMakerDelta::new(*maker);
        self.inner[self.len].write(new_delta);
        self.len += 1;

        // Return reference to the newly inserted delta
        Ok(unsafe { self.inner[self.len - 1].assume_init_mut() })
    }

    pub fn update_match<S: SideMarker>(
        &mut self,
        maker: &Address,
        lots: S::Lots,                                    // gained
        lots_opposite: <S::Opposite as SideMarker>::Lots, // lost
    ) -> Result<(), GoblinError> {
        let market_maker_delta = self.get_or_insert(maker)?;
        let deltas_for_side = S::maker_deltas_for_side(market_maker_delta);

        // Handling overflow
        //
        // Overflow is acceptable if it affects the maker but not solvency of the system.
        //
        // - free_lots_in will be credited to the maker. It is acceptable to overflow to 0.
        // - overflowing locked_lots_out should be unacceptable as it would lead to nothing deducted
        // from the maker for a match. However since resting orders are backed by maker's reserves,
        // it is not possible for the sum of quotes to overflow.
        deltas_for_side.free_lots_in += lots;
        deltas_for_side.locked_lots_out += lots_opposite;

        Ok(())
    }
}

pub struct MarketMakerDelta {
    pub address: Address,

    pub maker_deltas_for_bid: MakerDeltasForSide<Bid>,
    pub maker_deltas_for_ask: MakerDeltasForSide<Ask>,
}

impl MarketMakerDelta {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            maker_deltas_for_bid: MakerDeltasForSide::<Bid>::default(),
            maker_deltas_for_ask: MakerDeltasForSide::<Ask>::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct MakerDeltasForSide<S: SideMarker> {
    pub locked_lots_out: <S::Opposite as SideMarker>::Lots,
    pub free_lots_in: S::Lots,
}

impl<S: SideMarker> Default for MakerDeltasForSide<S> {
    fn default() -> Self {
        Self {
            locked_lots_out: <S::Opposite as SideMarker>::Lots::default(),
            free_lots_in: S::Lots::default(),
        }
    }
}
