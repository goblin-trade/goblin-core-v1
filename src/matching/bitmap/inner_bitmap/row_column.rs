use crate::matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RowColumn {
    pub row: Row,
    pub column: Column,
}

impl From<InnerPos> for RowColumn {
    fn from(value: InnerPos) -> Self {
        Self {
            row: Row::from(value),
            column: Column::from(value),
        }
    }
}
