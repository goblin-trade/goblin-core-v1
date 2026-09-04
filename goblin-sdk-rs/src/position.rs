use alloc::{collections::BTreeMap, vec::Vec};
use goblin_core_v1::quantities::{Column, InnerPos, OuterBitmapIndex, OuterPos, Position, Ticks};

use crate::{
    error::GoblinSdkError,
    types::{MakeAction, PositionedMakeOrder},
};

/// Extract (outer_bitmap_index, outer_pos, inner_pos) components from a strongly-typed `Position`
pub fn parts_from_position(position: Position) -> (OuterBitmapIndex, OuterPos, InnerPos) {
    let raw = position.inner;
    let outer_bitmap_index = OuterBitmapIndex::new(raw >> 16);
    let outer_pos = OuterPos::new(((raw >> 8) & 0xFF) as u8);
    let inner_pos = InnerPos::new((raw & 0xFF) as u8);
    (outer_bitmap_index, outer_pos, inner_pos)
}

/// Construct a strongly-typed `Position` from (outer_bitmap_index, outer_pos, inner_pos)
pub fn position_from_parts(
    outer_bitmap_index: OuterBitmapIndex,
    outer_pos: OuterPos,
    inner_pos: InnerPos,
) -> Position {
    let raw = ((outer_bitmap_index.inner) << 16)
        | ((outer_pos.inner as u64) << 8)
        | (inner_pos.inner as u64);
    Position::new(raw)
}

/// Extract Ticks from a `Position`
pub fn ticks_from_position(position: Position) -> Ticks {
    Ticks::new(position.inner >> 3)
}

/// Extract Column (0..7) from a `Position`
pub fn column_from_position(position: Position) -> Column {
    Column::new((position.inner & 0x07) as u8)
}

/// Construct a `Position` from Ticks and Column
pub fn position_from_ticks(ticks: Ticks, column: Column) -> Position {
    let raw = (ticks.inner << 3) | ((column.inner as u64) & 0x07);
    Position::new(raw)
}

/// Hierarchical group of make orders inside an inner bitmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerBitmapGroup {
    pub outer_pos: OuterPos,
    pub updates: Vec<(InnerPos, MakeAction)>,
}

/// Hierarchical group of inner bitmaps inside an outer bitmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OuterBitmapGroup {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub inner_bitmaps: Vec<InnerBitmapGroup>,
}

/// Group a list of positioned make orders into the 2-level bitmap hierarchy expected by goblin-core-v1.
pub fn group_makes_by_bitmaps(
    makes: &[PositionedMakeOrder],
) -> Result<Vec<OuterBitmapGroup>, GoblinSdkError> {
    if makes.is_empty() {
        return Ok(Vec::new());
    }

    // Group by outer_bitmap_index -> outer_pos -> updates
    let mut outer_map: BTreeMap<u64, BTreeMap<u8, Vec<(InnerPos, MakeAction)>>> = BTreeMap::new();

    for order in makes {
        let (outer_idx, outer_pos, inner_pos) = parts_from_position(order.position);
        outer_map
            .entry(outer_idx.inner)
            .or_default()
            .entry(outer_pos.inner)
            .or_default()
            .push((inner_pos, order.action));
    }

    if outer_map.len() > 3 {
        return Err(GoblinSdkError::OuterBitmapLimitExceeded(outer_map.len()));
    }

    let mut result = Vec::with_capacity(outer_map.len());
    for (outer_bitmap_raw, inner_map) in outer_map {
        if inner_map.len() > 255 {
            return Err(GoblinSdkError::InnerBitmapLimitExceeded(inner_map.len()));
        }

        let mut inner_bitmaps = Vec::with_capacity(inner_map.len());
        for (outer_pos_raw, updates) in inner_map {
            if updates.len() > 255 {
                return Err(GoblinSdkError::MakeUpdateLimitExceeded(updates.len()));
            }
            inner_bitmaps.push(InnerBitmapGroup {
                outer_pos: OuterPos::new(outer_pos_raw),
                updates,
            });
        }

        result.push(OuterBitmapGroup {
            outer_bitmap_index: OuterBitmapIndex::new(outer_bitmap_raw),
            inner_bitmaps,
        });
    }

    Ok(result)
}
