use crate::quantities::{FullPosition, PositionV2, TICK_POS, TickPos, Ticks};

impl From<TickPos> for Ticks {
    fn from(value: TickPos) -> Self {
        Self::new(value.inner)
    }
}

impl From<Ticks> for TickPos {
    fn from(value: Ticks) -> Self {
        Self::new(value.inner)
    }
}

impl From<PositionV2> for Ticks {
    fn from(value: PositionV2) -> Self {
        let tick_pos = value.extract_and_convert::<u64, TICK_POS>();
        Self::from(tick_pos)
    }
}
