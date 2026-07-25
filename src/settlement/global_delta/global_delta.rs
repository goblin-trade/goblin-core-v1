use crate::{
    axis::{
        leg::{Base, Quote},
        market::{LotSizePair, TokenIndexPair},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::{UnsideQuantity, ATOMS_PER_UNIT},
    settlement::{
        global_delta::{CounterpartyTokenKey, CounterpartyTriple, GlobalSender},
        local_delta::LocalDelta,
    },
    types::{Address, StoreReader},
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

        let atoms_per_lot_pair = ATOMS_PER_UNIT / lot_size_pair.unsided();
        let delta_atoms_per_lot_pair = atoms_per_lot_pair.try_into()?;

        // TODO code cleanup
        // * Macro to call function on both limbs
        // * Some way to connect B: TokenMarker to Base: LegMarker
        // * Pass token_index_pair to commit_size without increased generic count
        self.sender.commit_side::<B, Base>(
            base_token_index,
            &delta_atoms_per_lot_pair,
            local_delta,
        )?;
        self.sender.commit_side::<Q, Quote>(
            quote_token_index,
            &delta_atoms_per_lot_pair,
            local_delta,
        )?;

        for (counterparty, counterparty_pair) in local_delta.take.counterparties.into_iter() {
            self.counterparties.commit_side::<B, Base>(
                CounterpartyTokenKey {
                    counterparty: *counterparty,
                    token_index: base_token_index,
                },
                &atoms_per_lot_pair,
                counterparty_pair,
            )?;
            self.counterparties.commit_side::<Q, Quote>(
                CounterpartyTokenKey {
                    counterparty: *counterparty,
                    token_index: quote_token_index,
                },
                &atoms_per_lot_pair,
                counterparty_pair,
            )?;
        }
        Ok(())
    }

    pub fn settle(
        &self,
        trader: &Address,
        token_data_triple: &TokenDataTriple,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        self.sender
            .settle(trader, token_data_triple, msg_transfers)?;
        self.counterparties.settle(token_data_triple)?;
        Ok(())
    }
}
