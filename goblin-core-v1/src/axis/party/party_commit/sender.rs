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
    settlement::{global_delta::GlobalDelta, local_delta::LocalUpdate},
    types::StoreReader,
};

impl PartyCommit for Sender {
    fn commit_local_delta<'a, TP: TokenPair>(
        params: (&TokenIndexPair<TP>, &SamePair<UnsidedAtomsPerLot>),
        local_update: &LocalUpdate<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let sender_delta = Self::get(local_update);
        for_axes!(In => {
            Sender::commit_leg::<(TP, In)>(
                &(),
                params,
                sender_delta,
                global_delta
            )?;
        });

        Ok(())
    }
}
