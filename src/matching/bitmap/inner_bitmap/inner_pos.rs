use crate::{
    matching::bitmap::{row::Row, row_column::RowColumn, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct InnerPos {
    pub inner: u8,
}

impl InnerPos {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl Coordinate for InnerPos {
    type Inner = u8;
    const MIN: Self = Self::new(0);
    const MAX: Self = Self::new(u8::MAX);

    fn inner(self) -> Self::Inner {
        self.inner
    }
}

impl From<RowColumn> for InnerPos {
    fn from(value: RowColumn) -> Self {
        InnerPos::new(value.row.inner * 8 + value.column.inner)
    }
}

impl From<Ticks> for InnerPos {
    fn from(value: Ticks) -> Self {
        let row = Row::from(value);
        InnerPos::new(row.inner * 8)
    }
}
