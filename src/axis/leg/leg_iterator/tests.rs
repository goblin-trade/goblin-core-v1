use crate::{
    axis::leg::{leg_iterator::LegIterator, Base, Quote},
    matching::bitmap::{
        column::Column, compact_coordinates::CompactCoordinates,
        inner_coordinates::InnerCoordinates, row::Row,
    },
};

fn range_equal<In: LegIterator>(
    start: CompactCoordinates<In>,
    results_iterator: impl Iterator<Item = u8>,
) {
    let iterator = In::coordinates_iter(start);
    for (actual, expected) in iterator.zip(results_iterator) {
        assert_eq!(actual.inner, expected);
    }
}

#[test]
fn test_coordinates_iter_for_quote_in() {
    // Row 0, col 0
    range_equal::<Quote>(CompactCoordinates::new(0), 0..=255);

    // Row 0, last column
    range_equal::<Quote>(CompactCoordinates::new(7), 7..=255);

    // Row 1, col 0
    range_equal::<Quote>(CompactCoordinates::new(8), 8..=255);

    // Row 31, col 0
    range_equal::<Quote>(
        InnerCoordinates {
            row: Row::new(31),
            column: Column::new(0),
        }
        .into(),
        248..=255,
    );

    // Row 31, col 7 (last)
    range_equal::<Quote>(CompactCoordinates::new(255), 255..=255);
}

#[test]
fn test_coordinates_iter_for_base_in() {
    // Row 31, col 0 (the starting position)
    let start = CompactCoordinates::<Base>::from(InnerCoordinates {
        row: Row::new(31),
        column: Column::new(0),
    });
    let mut iterator = Base::coordinates_iter(start);

    for row in (0..=31).rev() {
        for column in 0..=7 {
            let coordinates = InnerCoordinates::from(iterator.next().unwrap());
            assert_eq!(coordinates.row.inner, row);
            assert_eq!(coordinates.column.inner, column);
        }
    }

    // Row 31, col 7 (last column of starting row)
    let start = CompactCoordinates::<Base>::from(InnerCoordinates {
        row: Row::new(31),
        column: Column::new(7),
    });
    let mut iterator = Base::coordinates_iter(start);
    let coordinates = InnerCoordinates::from(iterator.next().unwrap());
    assert_eq!(coordinates.row.inner, 31);
    assert_eq!(coordinates.column.inner, 7);

    for row in (0..=30).rev() {
        for column in 0..=7 {
            let coordinates = InnerCoordinates::from(iterator.next().unwrap());
            assert_eq!(coordinates.row.inner, row);
            assert_eq!(coordinates.column.inner, column);
        }
    }
}
