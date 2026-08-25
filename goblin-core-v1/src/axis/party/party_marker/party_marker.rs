use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        party::PartyEnum,
    },
    axis_helpers::AxisMarker,
    goblin_error::GoblinError,
    settlement::local_delta::LocalDelta,
    types::Address,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {
    fn add_local_delta<In: LegMatcher>(
        lots: In::Lots,
        lots_opposite: <In::Opposite as LegQuantities>::Lots,
        counterparty: &Address,
        local_delta: &mut LocalDelta,
    ) -> Result<(), GoblinError>;
}
