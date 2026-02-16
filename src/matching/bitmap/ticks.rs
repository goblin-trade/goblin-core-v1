use crate::{
    matching::bitmap::{OuterBitmapIndex, OuterPos, Row},
    quantities::Ticks,
};

impl From<Ticks> for OuterBitmapIndex {
    fn from(value: Ticks) -> Self {
        Self(value.inner / (256 * 32))
    }
}

impl From<Ticks> for OuterPos {
    fn from(value: Ticks) -> Self {
        Self(((value.inner / 32) % 256) as u8)
    }
}

impl From<Ticks> for Row {
    fn from(value: Ticks) -> Self {
        Self((value.inner % 32) as u8)
    }
}
