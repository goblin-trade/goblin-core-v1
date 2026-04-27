use crate::quantities::{Position, TickPosV2, Ticks};

impl From<TickPosV2> for Ticks {
    fn from(value: TickPosV2) -> Self {
        Self::new(value.inner)
    }
}

impl From<Ticks> for TickPosV2 {
    fn from(value: Ticks) -> Self {
        Self::new(value.inner)
    }
}

impl From<Position> for Ticks {
    fn from(value: Position) -> Self {
        let tick_pos = TickPosV2::from(value);
        Self::from(tick_pos)
    }
}
