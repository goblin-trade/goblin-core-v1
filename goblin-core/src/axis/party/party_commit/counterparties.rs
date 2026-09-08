use crate::{
    axis::party::{Counterparties, PartyCommit},
    axis_helpers::TokenPair,
    for_axes,
    goblin_error::GoblinError,
    market::CommonMarket,
    settlement::{global_delta::GlobalDelta, local_delta::LocalUpdate},
    types::StoreReader,
};

impl PartyCommit for Counterparties {
    fn commit<'a, TP: TokenPair>(
        market: &CommonMarket<TP>,
        local_update: &LocalUpdate<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let local_counterparties = &**Self::get_leg(local_update);
        for (address, local_counterparty) in local_counterparties.into_iter() {
            for_axes!(In => Self::commit_leg::<(TP, In)>(
                address,
                market,
                local_counterparty,
                global_delta
            )?);
        }

        Ok(())
    }
}
