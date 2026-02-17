use crate::{axis::leg::leg_matcher::LegMatcher, matching::bitmap::inner_pos::InnerPos};

pub struct Column {
    pub inner: u8,
}

impl Column {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl<In> From<InnerPos<In>> for Column
where
    In: LegMatcher,
{
    fn from(value: InnerPos<In>) -> Self {
        Column::new(value.inner % 8)
    }
}
