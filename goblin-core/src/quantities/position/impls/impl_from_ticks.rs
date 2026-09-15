use crate::quantities::{FullPos, FullPosition, TICK_POS, TickPos, Ticks};

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

impl From<FullPos> for Ticks {
    fn from(value: FullPos) -> Self {
        let tick_pos = value.extract_and_convert::<u64, TICK_POS>();
        Self::from(tick_pos)
    }
}
