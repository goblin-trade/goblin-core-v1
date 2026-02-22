use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, row::Row, Coordinate},
};

#[derive(Clone, Copy)]
pub struct InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub row: Row<In>,
    pub column: Column,
}

impl<In> InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub fn start_value() -> Self {
        Self {
            row: In::start_value(),
            column: Column::new(0),
        }
    }

    pub fn iter(self) -> impl Iterator<Item = Self> {
        let start_row = self.row;

        self.row.iter().flat_map(move |row| {
            let column_start = if row.inner == start_row.inner {
                self.column
            } else {
                Column::MIN
            };

            column_start
                .iter()
                .map(move |column| InnerCoordinates { row, column })
        })
    }
}
