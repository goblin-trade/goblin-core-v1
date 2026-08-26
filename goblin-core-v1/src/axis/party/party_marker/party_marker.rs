use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        party::PartyEnum,
        token::token_quantity::TokenQuantity,
    },
    axis_helpers::{AxisMarker, LegToToken},
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::GlobalDelta,
        local_delta::{LocalDelta, LocalDeposits},
        CheckedOps,
    },
    types::StoreReader,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {
    type Address;

    /// The local type that is converted into GlobalDeltaStore
    ///
    /// # Confusion
    ///
    /// This type doesn't have trait bound LocalDeltaStore
    type Local<'a, TP: TokenPair>;

    // TODO combine TP, In into wrapper trait with all bounds
    type GlobalDeltaStore<TP, In>: CheckedOps + Clone + Copy
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn try_new<'a, TP, In>(
        local: Self::Local<'a, TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn get_store<'a, TP, In>(
        address: &Self::Address,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn commit_local_delta<'a, TP>(
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        token_index_pair: &TokenIndexPair<TP>,
        local_delta: &LocalDelta<'a>,
        local_deposits: &LocalDeposits<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError>
    where
        TP: TokenPair;

    fn commit_leg<'a, TP, In>(
        address: &Self::Address,
        local: Self::Local<'a, TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let new_delta = Self::try_new::<TP, In>(local, atoms_per_lot_pair)?;
        let delta_store = Self::get_store(address, token_index_pair, global_delta)?;

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
