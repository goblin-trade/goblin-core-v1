use crate::matching::bitmap::{Column, InnerPos, Row};

pub struct InnerCoordinates {
    pub row: Row,
    pub column: Column,
}

impl From<InnerCoordinates> for InnerPos {
    fn from(value: InnerCoordinates) -> Self {
        InnerPos(value.row.0 * 8 + value.column.0)
    }
}
