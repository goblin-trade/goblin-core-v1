use crate::{
    axis::party::{Counterparties, PartyCommit, PartyEnum, PartySettle, Sender},
    axis_helpers::AxisMarker,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> + PartyCommit + PartySettle {}

impl PartyMarker for Sender {}
impl PartyMarker for Counterparties {}
