use crate::{
    axis::{
        leg::{Base, Quote, SamePair},
        market::{LotSizePair, TokenIndexPair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{CounterpartyTokenKey, CounterpartyTriple, GlobalSender},
        local_delta::LocalDelta,
    },
    types::StoreReader,
};

pub struct GlobalDelta {
    pub sender: GlobalSender,
    pub counterparties: CounterpartyTriple,
}

impl GlobalDelta {
    pub fn commit_local_delta<B, Q>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        lot_size_pair: &LotSizePair,
        local_delta: &LocalDelta,
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_token_index = Base::get(token_index_pair);
        let quote_token_index = Quote::get(token_index_pair);

        // TODO combine
        let atoms_per_lot_pair = &SamePair::<UnsidedAtomsPerLot>::from(lot_size_pair);
        let delta_atoms_per_lot_pair =
            &SamePair::<UnsidedDeltaAtomsPerLot>::try_from(lot_size_pair)?;

        self.sender.commit_side::<B, Base>(
            base_token_index,
            delta_atoms_per_lot_pair,
            local_delta,
        )?;
        self.sender.commit_side::<Q, Quote>(
            quote_token_index,
            delta_atoms_per_lot_pair,
            local_delta,
        )?;

        for (counterparty, counterparty_pair) in local_delta.take.counterparties.into_iter() {
            self.counterparties.commit_side::<B, Base>(
                CounterpartyTokenKey {
                    counterparty: *counterparty,
                    token_index: base_token_index,
                },
                delta_atoms_per_lot_pair,
                counterparty_pair,
            )?;
            self.counterparties.commit_side::<Q, Quote>(
                CounterpartyTokenKey {
                    counterparty: *counterparty,
                    token_index: quote_token_index,
                },
                delta_atoms_per_lot_pair,
                counterparty_pair,
            )?;
        }
        Ok(())
    }
}
