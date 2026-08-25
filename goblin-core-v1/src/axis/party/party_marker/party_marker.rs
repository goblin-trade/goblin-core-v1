use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
        party::PartyEnum,
        update::UpdateMarker,
    },
    axis_helpers::AxisMarker,
    goblin_error::GoblinError,
    quantities::{BaseLots, QuoteLots},
    settlement::local_delta::LocalDelta,
    types::Address,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {
    fn add_local_delta<UM: UpdateMarker, In: LegMatcher>(
        lots: In::Lots,
        counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError>;
}
