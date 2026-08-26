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
        deposits: &LocalDeposits<MS::Pair>,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        let market = &ctx.readables.market_readables().market;
        let atoms_per_lot_pair = ATOMS_PER_UNIT / market.lot_size_pair.unsided();

        let local_sender = Sender::get_leg(&ctx.writables.local_delta);
        for_axes!(In => {
            Sender::commit_leg::<MS::Pair, In>(
                &(),
                local_sender,
                deposits,
                &atoms_per_lot_pair,
                &market.token_index_pair,
                self
            )?;
        });

        let local_counterparties = &**Counterparties::get_leg(&ctx.writables.local_delta);
        for (address, local_counterparty) in local_counterparties.into_iter() {
            for_axes!(In => Counterparties::commit_leg::<MS::Pair, In>(
                address,
                local_counterparty,
                deposits,
                &atoms_per_lot_pair,
                &market.token_index_pair,
                self
            )?);
        }

        // // TODO reduce with for_axes!
        // // 1. Commit sender
        // let global_sender = Sender::get_leg_mut(self);

        // // Options
        // //
        // // 1. Trait with commit() function with `deposit` field. Rest remains symmetric.
        // //
        // // 2. commit_leg() is difficult because sender has extra `deposits` field while counterparty
        // // has extra counterparty `address` field

        // global_sender.commit::<MS::Pair>(
        //     local_sender,
        //     deposits, // TODO 1 field creating asymmetry
        //     &atoms_per_lot_pair,
        //     &market.token_index_pair,
        // )?;

        // 2. Commit counterparties
        // let global_counterparties = Counterparties::get_leg_mut(self);
        // let local_counterparties = &**Counterparties::get_leg(&ctx.writables.local_delta);

        // global_counterparties.commit::<MS::Pair>(
        //     local_counterparties,
        //     &market.token_index_pair,
        //     &atoms_per_lot_pair,
        // )?;

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
