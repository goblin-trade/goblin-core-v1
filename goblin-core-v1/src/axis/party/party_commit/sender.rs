use crate::{
    axis::{
        leg::SamePair,
        party::{PartyCommit, Sender},
    },
    axis_helpers::TokenPair,
    for_axes,
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::GlobalDelta,
        local_delta::{LocalDelta, LocalDeposits},
    },
    types::StoreReader,
};

impl PartyCommit for Sender {
    fn commit_local_delta<'a, TP: TokenPair>(
        params: (&TokenIndexPair<TP>, &SamePair<UnsidedAtomsPerLot>),
        (local_delta, local_deposits): (&LocalDelta<'a>, &LocalDeposits<TP>),
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let local_sender = Sender::get_leg(local_delta);
        for_axes!(In => {
            Sender::commit_leg::<(TP, In)>(
                &(),
                params,
                (local_sender, local_deposits),
                global_delta
            )?;
        });

        Ok(())
    }
}
