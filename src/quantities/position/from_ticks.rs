use crate::quantities::{Position, TickPos, Ticks};

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

impl From<Position> for Ticks {
    fn from(value: Position) -> Self {
        let tick_pos = TickPos::from(value);
        Self::from(tick_pos)
    }
}
