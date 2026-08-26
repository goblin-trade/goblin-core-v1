use crate::{
    axis::{
        party::{party_marker::party_marker::PartyMarker, Counterparties, Party, Sender},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::MarketSpec,
    for_axes,
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::{UnsideQuantity, ATOMS_PER_UNIT},
    settlement::{
        global_delta::{CounterpartyTriple, GlobalSender},
        local_delta::LocalDeposits,
    },
    types::{Address, StoreReader, Tuple},
    Ctx,
};

pub type GlobalDelta = Tuple<GlobalSender, CounterpartyTriple, Party>;

impl GlobalDelta {
    pub fn commit_local_delta<MS: MarketSpec>(
        &mut self,
        local_deposits: &LocalDeposits<MS::Pair>,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        let market = &ctx.readables.market_readables().market;
        let atoms_per_lot_pair = ATOMS_PER_UNIT / market.lot_size_pair.unsided();

        for_axes!(PT => {
            PT::commit_local_delta::<MS::Pair>(
                &atoms_per_lot_pair,
                &market.token_index_pair,
                &ctx.writables.local_delta,
                local_deposits,
                self
            )?;
        });

        // Reset counter of global mut counterparty buffer
        Counterparties::get_leg_mut(&mut ctx.writables.local_delta).reset();

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
        for_axes!(TM => Sender::get_leg(self).settle_leg::<TM>(
            recipient,
            token_data_triple,
            msg_transfers
        )?);
        for_axes!(TM => Counterparties::get_leg(self).settle_leg::<TM>(token_data_triple)?);

        Ok(())
    }
}
