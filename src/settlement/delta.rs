use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Quote},
        market::{market_marker::MarketMarker, LotSizePair, TokenIndexPair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::{GlobalDelta, GlobalMakerDeltas, MakerDeltaKey},
        local_delta::LocalDelta,
        CheckedAdd, ConstZero, SidedTakeDeltaPairV2, SidedTakeDeltaV2, UnsideDelta,
        UnsidedTakeDeltaV2,
    },
    types::StoreReader,
};

/// Global static mut Delta, initially zero filled.
///
/// `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut DELTA: Delta = Delta::ZEROED;

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl ConstZero for Delta {
    const ZEROED: Self = Self {
        global: GlobalDelta::ZEROED,
        local: LocalDelta::ZEROED,
    };
}

impl Delta {
    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA }
    }

    pub fn commit_local_delta<M, B, Q>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_token_index = Base::get(token_index_pair);
        let quote_token_index = Quote::get(token_index_pair);

        B::commit_sender_delta::<Base>(base_token_index, lot_size_pair, self)?;
        Q::commit_sender_delta::<Quote>(quote_token_index, lot_size_pair, self)?;

        for (maker, delta_pair) in self.local.local_maker_deltas.iter() {
            Self::commit_maker_delta::<B, Base>(
                MakerDeltaKey {
                    maker: *maker,
                    token_index: base_token_index,
                },
                delta_pair,
                lot_size_pair,
                &mut self.global.maker_deltas,
            )?;

            Self::commit_maker_delta::<Q, Quote>(
                MakerDeltaKey {
                    maker: *maker,
                    token_index: quote_token_index,
                },
                delta_pair,
                lot_size_pair,
                &mut self.global.maker_deltas,
            )?;
        }

        // // Reset local delta for reuse
        // self.local.deposits.reset::<B, Q>();
        Ok(())
    }

    pub fn commit_maker_delta<T, In>(
        maker_delta_key: MakerDeltaKey<T>,
        delta_pair: &SidedTakeDeltaPairV2,
        lot_size_pair: &LotSizePair,
        global_maker_deltas: &mut GlobalMakerDeltas,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
        SidedTakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedTakeDeltaV2>,
    {
        let global_maker_delta = T::get_leg_mut(global_maker_deltas);

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
