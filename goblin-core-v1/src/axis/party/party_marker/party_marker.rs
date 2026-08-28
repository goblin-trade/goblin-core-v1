use crate::{
    axis::party::{Counterparties, PartyCommit, PartyEnum, Sender},
    axis_helpers::AxisMarker,
};

pub trait PartyMarker: AxisMarker<Enum = PartyEnum> + PartyCommit {}

impl PartyMarker for Sender {}
impl PartyMarker for Counterparties {}
