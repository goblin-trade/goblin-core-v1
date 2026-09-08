use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker, HardcodedCallerList},
        party::{Counterparties, PartySettle},
        token::{token_marker::TokenMarker, token_reader::TokenDataTriple},
    },
    goblin_error::GoblinError,
    input_processor::{CallerAddresses, MsgTransfers},
    match_axes,
    settlement::global_delta::{CounterpartyMap, GlobalDelta, TokenDelta},
    types::StoreReader,
};

impl PartySettle for Counterparties {
    fn settle<'a, TM>(
        global_delta: &'a GlobalDelta,
        token_data_triple: &TokenDataTriple<'a>,
        _msg_transfers: &MsgTransfers,
        _caller_addresses: CallerAddresses<'a>,
    ) -> Result<(), GoblinError>
    where
        TM: TokenMarker,
        &'a TM::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<TM>>,
    {
        let counterparties_delta = Counterparties::get_leg(global_delta);

        let counterparty_map: &CounterpartyMap<TM> = TM::get_leg(counterparties_delta);

        for (counterparty_key, counterparty) in counterparty_map.into_iter() {
            let maybe_hardcoded_caller_index =
                HardcodedCallerList::index(&counterparty_key.counterparty);
            let caller_enum = CallerEnum::from(maybe_hardcoded_caller_index);

            match_axes!(CM = caller_enum => {
                let locator = CM::get_locator(maybe_hardcoded_caller_index);

                let store_hash = counterparty_key.get_store_hash::<CM>(locator, token_data_triple);
                let mut store = store_hash.load();

                store.update_counterparty(counterparty)?;
                store_hash.store(&store);
            });
        }

        Ok(())
    }
}
