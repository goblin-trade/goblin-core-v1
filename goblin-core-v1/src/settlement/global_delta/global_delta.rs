use crate::{
    axis::{
        party::{Counterparties, Party, PartyCommit, Sender},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::MarketSpec,
    for_axes,
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    market::CommonMarket,
    quantities::{UnsideQuantity, ATOMS_PER_UNIT},
    settlement::{
        global_delta::{CounterpartyTriple, GlobalSender},
        local_delta::LocalUpdate,
    },
    types::{Address, StoreReader, Tuple},
};

pub type GlobalDelta = Tuple<GlobalSender, CounterpartyTriple, Party>;

impl GlobalDelta {
    pub fn commit<MS: MarketSpec>(
        &mut self,
        market: &CommonMarket<MS::Pair>,
        local_update: &LocalUpdate<MS::Pair>,
    ) -> Result<(), GoblinError> {
        let atoms_per_lot_pair = ATOMS_PER_UNIT / market.lot_size_pair.unsided();

        for_axes!(PT => {
            PT::commit_local_delta::<MS::Pair>(
                (&market.token_index_pair, &atoms_per_lot_pair),
                local_update,
                self
            )?;
        });

        Ok(())
    }

    pub fn settle(
        &self,
        recipient: &Address,
        token_data_triple: &TokenDataTriple,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        // TODO trait on GlobalSender and CounterpartyTriple with
        // commit and settle function
        //
        // We can combine it into a single for_axes!(|TM, TR|)
        //
        for_axes!(TM0 => Sender::get_leg(self).settle_leg::<TM0>(
            recipient,
            token_data_triple,
            msg_transfers
        )?);
        for_axes!(TM0 => Counterparties::get_leg(self).settle_leg::<TM0>(token_data_triple)?);

        Ok(())
    }
}
