use crate::{
    axis::{
        party::{Counterparties, PartySettle},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::global_delta::{CounterpartyMap, GlobalDelta, TokenDelta},
    types::{Address, StoreReader},
};

impl PartySettle for Counterparties {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        _trader: &Address,
        _transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
    {
        let counterparties_delta = Counterparties::get_leg(global_delta);

        let counterparty_map: &CounterpartyMap<TM> = TM::get_leg(counterparties_delta);

        for (counterparty_key, counterparty) in counterparty_map.into_iter() {
            let store_hash = counterparty_key.get_store_hash(token_data_triple);
            let mut store = store_hash.load();
            store.update_counterparty(counterparty)?;
            store_hash.store(&store);
        }

        Ok(())
    }
}
