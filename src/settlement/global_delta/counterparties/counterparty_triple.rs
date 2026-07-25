use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_marker::TokenMarker, token_reader::TokenDataTriple, CustomERC20, HardcodedERC20,
            Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTokenKey},
        local_delta::local_take::LocalCounterparty,
        CheckedOps,
    },
    types::Triple,
};

pub type CounterpartyTriple = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;

impl CounterpartyTriple {
    pub fn commit_side<T, In>(
        &mut self,
        key: CounterpartyTokenKey<T>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        local_counterparty_pair: &SamePair<LocalCounterparty>,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let local_counterparty = In::get(local_counterparty_pair);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let atoms_pair = atoms_per_lot * local_counterparty;

        let global_counterparty = T::get_leg_mut(self)
            .get_or_insert_mut(key)
            .ok_or(GoblinError::GlobalCounterpartyFull)?;

        *global_counterparty = global_counterparty
            .checked_add(atoms_pair)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    pub fn settle(&self, token_data_triple: &TokenDataTriple) -> Result<(), GoblinError> {
        self.settle_leg::<ETH>(token_data_triple)?;
        self.settle_leg::<HardcodedERC20>(token_data_triple)?;
        self.settle_leg::<CustomERC20>(token_data_triple)?;
        Ok(())
    }

    fn settle_leg<T>(&self, token_data_triple: &TokenDataTriple) -> Result<(), GoblinError>
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
