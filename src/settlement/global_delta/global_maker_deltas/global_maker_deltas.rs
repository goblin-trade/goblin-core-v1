use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::{MakerDeltaKey, MakerDeltaMap},
        CheckedAdd, ConstZero, SidedTakeDeltaPairV2, SidedTakeDeltaV2, UnsideDelta,
        UnsidedTakeDeltaV2,
    },
    types::Triple,
};

/// Global deltas of makers that matched against msg.sender
pub type GlobalMakerDeltas =
    Triple<MakerDeltaMap<ETH>, MakerDeltaMap<HardcodedERC20>, MakerDeltaMap<CustomERC20>, Token>;

impl ConstZero for GlobalMakerDeltas {
    const ZEROED: Self = Self::new(
        MakerDeltaMap::ZEROED,
        MakerDeltaMap::ZEROED,
        MakerDeltaMap::ZEROED,
    );
}

impl GlobalMakerDeltas {
    pub fn commit<T, In>(
        &mut self,
        maker_delta_key: MakerDeltaKey<T>,
        delta_pair: &SidedTakeDeltaPairV2,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
        SidedTakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedTakeDeltaV2>,
    {
        let global_maker_delta = T::get_leg_mut(self);

        // Namespaced by- maker > leg > take_in/take_out
        let maker_store = global_maker_delta
            .get_or_insert_mut(maker_delta_key)
            .ok_or(GoblinError::GlobalMakerListFull)?;

        let sided_delta = In::get_leg(delta_pair);
        let unsided_delta = sided_delta.unside(lot_size_pair);

        *maker_store = maker_store
            .checked_add(unsided_delta)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
