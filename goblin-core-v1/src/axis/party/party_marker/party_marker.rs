use crate::{axis::party::PartyEnum, axis_helpers::AxisMarker};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> {}
