use crate::{
    axis::party::{PartyCommit, Sender},
    axis_helpers::TokenPair,
    for_axes,
    goblin_error::GoblinError,
    market::CommonMarket,
    settlement::{global_delta::GlobalDelta, local_delta::LocalUpdate},
    types::StoreReader,
};

impl PartyCommit for Sender {
    fn commit<'a, TP: TokenPair>(
        market: &CommonMarket<TP>,
        local_update: &LocalUpdate<TP>,
        global_delta: &mut GlobalDelta,
    ) -> Result<(), GoblinError> {
        let sender_delta = Self::get(local_update);
        for_axes!(In => {
            Sender::commit_leg::<(TP, In)>(
                &(),
                market,
                sender_delta,
                global_delta
            )?;
        });

        Ok(())
    }
}
