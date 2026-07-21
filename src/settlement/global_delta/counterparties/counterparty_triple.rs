use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTokenKey},
        local_delta::DeltaLotsPair,
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
        delta_lots_pair: &DeltaLotsPair,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let delta_lots = In::get(delta_lots_pair);
        let atoms_per_lot = In::get(atoms_per_lot_pair);
        let delta_atoms = delta_lots * atoms_per_lot;

        let counterparty_map = T::get_leg_mut(self);

        let net_delta = counterparty_map
            .get_or_insert_mut(key)
            .ok_or(GoblinError::GlobalMakerListFull)?;

        *net_delta = net_delta
            .checked_add(delta_atoms)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
