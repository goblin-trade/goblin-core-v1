use core::todo;

use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    axis_helpers::LegToToken,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::{UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{GlobalDeltaStore, TokenDelta},
        local_delta::{LocalDeposits, LocalSender},
    },
    types::StoreReader,
};

impl<T> GlobalDeltaStore for TokenDelta<T>
where
    T: TokenMarker,
{
    type LocalDeltaStore = LocalSender;

    fn try_new<TP, In>(
        local_delta: &Self::LocalDeltaStore,
        local_deposits: &LocalDeposits<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let local_deposit = In::get(local_deposits);
        let delta_atoms_per_lot_pair: SamePair<UnsidedDeltaAtomsPerLot> =
            atoms_per_lot_pair.try_into()?;

        let atoms_per_lot = In::get(&delta_atoms_per_lot_pair);

        todo!()
        // error
        // expected <T as TokenQuantity>::LocalDeposit, found <<In as LegToToken<TP>>::Selected as TokenQuantity>::LocalDeposit
        // let deposit = T::get_global_deposit(local_deposit, atoms_per_lot);

        // // TODO checked mul?
        // let local_take = In::get(&local_delta.take.inner);
        // let take = local_take * atoms_per_lot;

        // let local_make = In::get(&local_delta.make.inner);
        // let make = local_make * atoms_per_lot;

        // Ok(Self {
        //     deposit,
        //     take,
        //     make,
        // })
    }
}
