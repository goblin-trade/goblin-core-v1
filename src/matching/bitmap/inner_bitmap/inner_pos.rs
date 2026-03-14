use core::marker::PhantomData;

use crate::{
    axis::leg::{leg_iterator::LegIterator, leg_matcher::LegMatcher},
    matching::bitmap::{row::Row, row_column::RowColumn, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct InnerPos<In>
where
    In: LegIterator,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> InnerPos<In>
where
    In: LegIterator,
{
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn adjust_start(self, on_start: bool) -> Self {
        if on_start {
            self
        } else {
            In::start_value()
        }
    }

    pub fn adjust_limit(self, on_limit: bool) -> Self {
        if on_limit {
            self
        } else {
            In::end_value()
        }
    }
}

impl<In> Coordinate for InnerPos<In>
where
    In: LegIterator,
{
    type Inner = u8;
    const MIN: Self = Self::new(0);
    const MAX: Self = Self::new(u8::MAX);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<RowColumn<In>> for InnerPos<In>
where
    In: LegMatcher,
{
    fn from(value: RowColumn<In>) -> Self {
        InnerPos::new(value.row.inner * 8 + value.column.inner)
    }
}

impl<In> From<Ticks> for InnerPos<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        let row = Row::<In>::from(value);
        InnerPos::new(row.inner * 8)
    }
}
