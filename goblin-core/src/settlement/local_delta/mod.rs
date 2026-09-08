pub mod local_counterparties;
pub mod local_delta_store;
pub mod local_deposits;
pub mod local_sender;
pub mod local_update;

pub use local_counterparties::*;
pub use local_delta_store::*;
pub use local_deposits::*;
pub use local_sender::*;
pub use local_update::*;

mod impl_from_local_delta;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        party::{Counterparties, Party, Sender},
    },
    goblin_error::GoblinError,
    matching::{FillOutcome, MatchDelta},
    quantities::BaseLotsPerBaseUnit,
    types::{StoreReader, Tuple},
};

pub type LocalDelta<'a> = Tuple<LocalSender, &'a mut LocalCounterparties, Party>;

impl<'a> LocalDelta<'a> {
    pub fn add_take<In: LegMatcher>(
        &mut self,
        base_lot_size: BaseLotsPerBaseUnit,
        fill_outcome: &FillOutcome<In>,
    ) -> Result<(), GoblinError> {
        let match_delta = MatchDelta::<In> {
            matching_lots: fill_outcome.matched,
            base_lot_size,
            price_in_quote_lots: fill_outcome.price_in_quote_lots,
        };
        let lots = match_delta.lots();
        let lots_opposite = match_delta.lots_opposite()?;

        let sender = Sender::get_leg_mut(self);
        sender.take.add::<In>(lots, lots_opposite)?;

        let counterparty = Counterparties::get_leg_mut(self)
            .get_or_insert_mut(*fill_outcome.counterparty)
            .ok_or(GoblinError::LocalCounterpartyFull)?;

        counterparty.add::<In>(lots, lots_opposite)?;

        Ok(())
    }
}
