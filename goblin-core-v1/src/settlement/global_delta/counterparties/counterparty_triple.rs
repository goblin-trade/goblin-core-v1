use crate::{
    axis::token::{
        token_marker::TokenMarker, token_reader::TokenDataTriple, CustomERC20, HardcodedERC20,
        Token, ETH,
    },
    goblin_error::GoblinError,
    settlement::global_delta::CounterpartyMap,
    types::Triple,
};

pub type CounterpartyTriple = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;

impl CounterpartyTriple {
    pub fn settle_leg<T>(&self, token_data_triple: &TokenDataTriple) -> Result<(), GoblinError>
    where
        T: TokenMarker,
    {
        let counterparty_map: &CounterpartyMap<T> = T::get_leg(self);

        for (counterparty_key, counterparty) in counterparty_map.into_iter() {
            let store_hash = counterparty_key.get_store_hash(token_data_triple);
            let mut store = store_hash.load();
            store.update_counterparty(counterparty)?;
            store_hash.store(&store);
        }
        Ok(())
    }
}
