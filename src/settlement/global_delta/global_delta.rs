use crate::{
    axis::{
        market::{token_pair::TokenPair, LotSizePair, TokenIndexPair},
        token::token_reader::TokenDataTriple,
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::{UnsideQuantity, ATOMS_PER_UNIT},
    settlement::{
        global_delta::{CounterpartyTriple, GlobalSender},
        local_delta::LocalDelta,
    },
    types::Address,
};

pub struct GlobalDelta {
    pub sender: GlobalSender,
    pub counterparties: CounterpartyTriple,
}

impl GlobalDelta {
    pub fn commit_local_delta<TP: TokenPair>(
        &mut self,
        token_index_pair: &TokenIndexPair<TP>,
        lot_size_pair: &LotSizePair,
        local_delta: &LocalDelta,
    ) -> Result<(), GoblinError> {
        let atoms_per_lot_pair = ATOMS_PER_UNIT / lot_size_pair.unsided();

        for_axes!(|In| self.sender.commit_leg::<TP, In>(
            local_delta,
            token_index_pair,
            &atoms_per_lot_pair,
        )?);

        for counterparty_data in local_delta.take.counterparties.into_iter() {
            for_axes!(|In| self.counterparties.commit_leg::<TP, In>(
                counterparty_data,
                token_index_pair,
                &atoms_per_lot_pair,
            )?);
        }
        Ok(())
    }

    pub fn settle(
        &self,
        trader: &Address,
        token_data_triple: &TokenDataTriple,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        // TODO convert sender and counterparty to new axis TR = Trader
        // This way both global and local deltas can become tuples
        // we will have uniform function API
        //
        // Sender needs trader and msg_transfers but counterparty doesn't.
        //
        // We can combine it into a single for_axes!(|TM, TR|)
        //
        for_axes!(|TM| self
            .sender
            .settle_leg::<TM>(trader, token_data_triple, msg_transfers)?);
        for_axes!(|TM| self.counterparties.settle_leg::<TM>(token_data_triple)?);

        Ok(())
    }
}
