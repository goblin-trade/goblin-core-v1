use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
        market::{market_marker::MarketMarker, LotSizePair, TokenIndexPair},
        token::{token_marker::TokenMarker, ETH},
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::{GlobalDelta, GlobalMakerDeltas, MakerDeltaKey},
        local_delta::LocalDelta,
        ConstZero, SidedTakeDeltaPairV2,
    },
    types::{Address, StoreReader},
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
        // 1. Sender- deposits, taker delta, maker delta
        // need to call 3 times for ETH, hardcoded token and custom token

        B::add_delta::<Base>(self, Base::get(token_index_pair), lot_size_pair)?;
        Q::add_delta::<Quote>(self, Quote::get(token_index_pair), lot_size_pair)?;

        // Maker deltas

        let global_maker_delta = B::get_leg_mut(&mut self.global.maker_deltas);

        // let global_maker_delta = B::get_leg_mut(&mut self.global.maker_deltas);

        for (maker, delta_pair) in self.local.local_maker_deltas.iter() {
            Self::commit_maker_delta::<B, Base>(
                *maker,
                delta_pair,
                Base::get(token_index_pair),
                lot_size_pair,
                &mut self.global.maker_deltas,
            )?;

            // let maker_delta_key = MakerDeltaKey::<B> {
            //     maker: *maker,
            //     token_index: Base::get(token_index_pair),
            // };

            // // Namespaced by- maker > leg > take_in/take_out
            // let maker_store = global_maker_delta
            //     .get_or_insert_mut(maker_delta_key)
            //     .ok_or(GoblinError::GlobalMakerListFull)?;

            // let maker_delta = In::get_leg(delta_pair);
            // let maker_delta_unsided = maker_delta.unside(lot_size_pair);

            // *maker_store = maker_store
            //     .checked_add(maker_delta_unsided)
            //     .ok_or(GoblinError::Overflow)?;
        }

        // // Reset local delta for reuse
        // self.local.deposits.reset::<B, Q>();
        Ok(())
    }

    pub fn commit_maker_delta<T, In>(
        // &mut self,
        maker: Address,
        delta_pair: &SidedTakeDeltaPairV2,
        token_index: T::TokenIndex,
        lot_size_pair: &LotSizePair,
        global_maker_deltas: &mut GlobalMakerDeltas,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let maker_delta_key = MakerDeltaKey::<T> { maker, token_index };

        let global_maker_delta = T::get_leg_mut(global_maker_deltas);

        // Namespaced by- maker > leg > take_in/take_out
        let maker_store = global_maker_delta
            .get_or_insert_mut(maker_delta_key)
            .ok_or(GoblinError::GlobalMakerListFull)?;

        Ok(())
    }
}
