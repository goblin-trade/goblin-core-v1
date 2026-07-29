use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, Pair, SamePair},
        market::TokenIndexPair,
        token::{
            token_marker::TokenMarker, token_quantity::TokenQuantity,
            token_reader::TokenDataTriple, CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTokenKey},
        local_delta::local_take::LocalCounterparty,
        CheckedOps,
    },
    types::{Address, StoreReader, Triple},
};

pub type CounterpartyTriple = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;

impl CounterpartyTriple {
    pub fn commit_leg<B, Q, In>(
        &mut self,
        counterparty_data: &(Address, SamePair<LocalCounterparty>),
        token_index_pair: &TokenIndexPair<B, Q>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher
            + LegToToken<Pair<B, Q>>
            + StoreReader<TokenIndexPair<B, Q>, Result = <In::Selected as TokenQuantity>::TokenIndex>,
    {
        let local_counterparty = In::get(&counterparty_data.1);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        let global_counterparty = In::Selected::get_leg_mut(self)
            .get_or_insert_mut(CounterpartyTokenKey {
                counterparty: counterparty_data.0,
                token_index: In::get(token_index_pair),
            })
            .ok_or(GoblinError::GlobalCounterpartyFull)?;

        *global_counterparty = global_counterparty
            .checked_add(atoms_pair)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    pub fn settle_leg<T>(&self, token_data_triple: &TokenDataTriple) -> Result<(), GoblinError>
    where
        T: TokenMarker,
    {
        let counterparty_map = T::get_leg(self);

        for (counterparty_key, counterparty) in counterparty_map.into_iter() {
            let store_hash = counterparty_key.get_store_hash(token_data_triple);
            let mut store = store_hash.load();
            store.update_counterparty(counterparty)?;
            store_hash.store(&store);
        }
        Ok(())
    }
}
