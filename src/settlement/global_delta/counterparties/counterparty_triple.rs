use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_marker::TokenMarker,
            token_reader::{token_data_triple, TokenDataTriple},
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTokenKey},
        local_delta::{local_take::LocalCounterparty, DeltaLotsPair},
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
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_counterparty_pair: &SamePair<LocalCounterparty>,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let local_counterparty = In::get(local_counterparty_pair);
        let atoms_per_lot = In::get(atoms_per_lot_pair);

        // // first we need unsided atoms_per_lot_pair
        // let atoms_pair = local_counterparty * atoms_per_lot;

        // let counterparty_map = T::get_leg_mut(self);

        // let global_counterparty = counterparty_map
        //     .get_or_insert_mut(key)
        //     .ok_or(GoblinError::GlobalCounterpartyFull)?;

        // *global_counterparty = global_counterparty
        //     .checked_add(atoms_pair)
        //     .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    fn settle_token<T>(&self, token_data_triple: &TokenDataTriple) -> Result<(), GoblinError>
    where
        T: TokenMarker,
    {
        let counterparty_map = T::get_leg(self);

        for (counterparty_key, net_delta) in counterparty_map.into_iter() {
            let store_hash = counterparty_key.get_store_hash(token_data_triple);

            let mut store = store_hash.load();

            // Working of counterparties
            //
            // * Update both base and quote token for counterparty
            // * Counterparty gains 'In' token. Add to free.
            // * Counterparty loses 'In::Opposite'. Subtract from free.
            //
            // Counterparty delta has no concept of negative?
        }

        Ok(())
    }
}
