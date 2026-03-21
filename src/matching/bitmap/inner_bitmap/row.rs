use crate::{
    matching::bitmap::{inner_pos::InnerPos, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Row {
    pub inner: u8,
}

impl Row {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl Coordinate for Row {
    type Inner = u8;
    const MIN: Self = Row::new(0);
    const MAX: Self = Row::new(31);

    fn inner(self) -> Self::Inner {
        self.inner
    }
}

impl From<InnerPos> for Row {
    fn from(value: InnerPos) -> Self {
        Row::new(value.inner / 8)
    }
}

impl From<Ticks> for Row {
    fn from(value: Ticks) -> Self {
        Self::new((value.inner % 32) as u8)
    }
}
