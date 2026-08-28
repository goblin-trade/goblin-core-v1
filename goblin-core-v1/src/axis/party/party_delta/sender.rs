use crate::{
    axis::{
        leg::SamePair,
        party::{PartyDelta, Sender},
        token::token_marker::TokenMarker,
    },
    axis_helpers::{PairLeg, TokenPair},
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::{UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot},
    settlement::{
        global_delta::{GlobalDelta, TokenDelta},
        local_delta::{LocalDeposits, LocalSender},
    },
    types::StoreReader,
};

impl PartyDelta for Sender {
    type Address = ();

    type Local<'a, TP: TokenPair> = (&'a LocalSender, &'a LocalDeposits<TP>);
    type GlobalDeltaStore<PL: PairLeg> = TokenDelta<PL::Selected>;

    fn try_new<'a, PL: PairLeg>(
        (local_delta, local_deposits): Self::Local<'a, PL::Pair>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<Self::GlobalDeltaStore<PL>, GoblinError> {
        let local_deposit = PL::Leg::get(local_deposits);
        let delta_atoms_per_lot_pair =
            SamePair::<UnsidedDeltaAtomsPerLot>::try_from(atoms_per_lot_pair)?;

        let atoms_per_lot = PL::Leg::get(&delta_atoms_per_lot_pair);

        let deposit = PL::Selected::get_global_deposit(local_deposit, atoms_per_lot);
        let local_take = PL::Leg::get(&local_delta.take.inner);

        // TODO checked mul?
        let take = local_take * atoms_per_lot;

        let local_make = PL::Leg::get(&local_delta.make.inner);
        let make = local_make * atoms_per_lot;

        Ok(TokenDelta {
            deposit,
            take,
            make,
        })
    }

    fn get_store<'a, PL: PairLeg>(
        _address: &Self::Address,
        token_index_pair: &TokenIndexPair<PL::Pair>,
        global_delta: &'a mut GlobalDelta,
    ) -> Result<&'a mut Self::GlobalDeltaStore<PL>, GoblinError> {
        let sender_delta = Sender::get_leg_mut(global_delta);
        let deltas_list = PL::Selected::get_leg_mut(sender_delta);
        let token_index = PL::Leg::get(token_index_pair);

        Ok(&mut deltas_list[token_index])
    }
}
