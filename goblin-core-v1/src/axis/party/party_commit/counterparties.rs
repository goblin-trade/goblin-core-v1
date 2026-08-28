use crate::{
    axis::{
        leg::SamePair,
        party::{Counterparties, PartyCommit},
    },
    axis_helpers::TokenPair,
    for_axes,
    goblin_error::GoblinError,
    market::TokenIndexPair,
    quantities::UnsidedAtomsPerLot,
    settlement::{global_delta::GlobalDelta, local_delta::LocalUpdateV2},
    types::StoreReader,
};

impl PartyCommit for Counterparties {
    fn commit_local_delta<'a, TP: TokenPair>(
        params: (&TokenIndexPair<TP>, &SamePair<UnsidedAtomsPerLot>),
        local_update: &LocalUpdateV2<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let local_counterparties = &**Self::get_leg(local_update);
        for (address, local_counterparty) in local_counterparties.into_iter() {
            for_axes!(In => Self::commit_leg::<(TP, In)>(
                address,
                params,
                &local_counterparty,
                global_delta
            )?);
        }

        Ok(())
    }
}
