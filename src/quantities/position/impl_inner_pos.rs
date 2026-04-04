use crate::quantities::{ColumnV2, InnerPosV2, RowV2};

impl From<InnerPosV2> for RowV2 {
    fn from(value: InnerPosV2) -> Self {
        Self::new(value.inner / 8)
    }
}

impl From<InnerPosV2> for ColumnV2 {
    fn from(value: InnerPosV2) -> Self {
        Self::new(value.inner % 8)
    }
}
