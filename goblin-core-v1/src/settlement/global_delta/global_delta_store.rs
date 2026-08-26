use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDeposits, CheckedOps},
    types::{Address, StoreReader},
};

pub trait GlobalDeltaStore<TP, In>: Sized + Clone + Copy + CheckedOps
where
    TP: TokenPair,
    In: LegMatcher
        + LegToToken<TP>
        + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
        + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
{
    type LocalDeltaStore;

    fn try_new(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError>;

    fn get_store<'a>(
        address: &Address,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self, GoblinError>;

    fn commit_leg(
        address: &Address,
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let new_delta = Self::try_new(local_delta, local_deposits, atoms_per_lot_pair)?;
        let delta_store = Self::get_store(address, token_index_pair, global_delta)?;

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
