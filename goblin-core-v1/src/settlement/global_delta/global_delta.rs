use crate::{
    axis::{
        party::{Counterparties, Party, Sender},
        token::token_reader::TokenDataTriple,
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::global_delta::{CounterpartyTriple, GlobalSender},
    types::{Address, StoreReader, Tuple},
};

pub type GlobalDelta = Tuple<GlobalSender, CounterpartyTriple, Party>;

impl GlobalDelta {
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
