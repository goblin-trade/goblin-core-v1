use crate::matching::bitmap::{inner_pos::InnerPos, Coordinate};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Column {
    pub inner: u8,
}

impl Column {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl Coordinate for Column {
    type Inner = u8;

    const MIN: Self = Self::new(0);
    const MAX: Self = Self::new(7);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    // fn closer_to_centre(self, other: Self) -> bool {
    //     // Always move left to right for column
    //     // The column with lower index is popped first
    //     self < other
    // }
}

impl From<InnerPos> for Column {
    fn from(value: InnerPos) -> Self {
        Column::new(value.inner % 8)
    }
}
