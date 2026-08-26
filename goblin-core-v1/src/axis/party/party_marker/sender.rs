use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        party::{party_marker::party_marker::PartyMarker, Sender},
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    axis_helpers::LegToToken,
    for_axes,
    goblin_error::GoblinError,
    market::{TokenIndexPair, TokenPair},
    quantities::{UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        local_delta::{LocalDelta, LocalDeposits, LocalSender},
    },
    types::StoreReader,
};

impl PartyMarker for Sender {
    type Address = ();
    type Local<'a, TP: TokenPair> = (&'a LocalSender, &'a LocalDeposits<TP>);

    type GlobalDeltaStore<TP, In>
        = TokenDelta<In::Selected>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>;

    fn try_new<'a, TP, In>(
        (local_delta, local_deposits): Self::Local<'a, TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let local_deposit = In::get(local_deposits);
        let delta_atoms_per_lot_pair =
            SamePair::<UnsidedDeltaAtomsPerLot>::try_from(atoms_per_lot_pair)?;

        let atoms_per_lot = In::get(&delta_atoms_per_lot_pair);

        let deposit = In::Selected::get_global_deposit(local_deposit, atoms_per_lot);
        let local_take = In::get(&local_delta.take.inner);

        // TODO checked mul?
        let take = local_take * atoms_per_lot;

        let local_make = In::get(&local_delta.make.inner);
        let make = local_make * atoms_per_lot;

        Ok(TokenDelta {
            deposit,
            take,
            make,
        })
    }

    fn get_store<'a, TP, In>(
        _address: &Self::Address,
        token_index_pair: &TokenIndexPair<TP>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<TP, In>, GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
            + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let sender_delta = Sender::get_leg_mut(global_delta);
        let deltas_list = In::Selected::get_leg_mut(sender_delta);
        let token_index = In::get(token_index_pair);

        Ok(&mut deltas_list[token_index])
    }

    fn commit_local_delta<'a, TP: TokenPair>(
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
        token_index_pair: &TokenIndexPair<TP>,
        local_delta: &LocalDelta<'a>,
        local_deposits: &LocalDeposits<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let local_sender = Sender::get_leg(local_delta);
        for_axes!(In => {
            Sender::commit_leg::<TP, In>(
                &(),
                (local_sender, local_deposits),
                atoms_per_lot_pair,
                token_index_pair,
                global_delta
            )?;
        });

        Ok(())
    }
}
