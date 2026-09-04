use crate::{
    error::GoblinSdkError,
    types::{MakeAction, PositionedMakeOrder},
};
use std::collections::BTreeMap;

/// Extract (outer_bitmap_index, outer_pos, inner_pos) components from a 64-bit Position
pub const fn parts_from_position(position: u64) -> (u64, u8, u8) {
    let outer_bitmap_index = position >> 16;
    let outer_pos = ((position >> 8) & 0xFF) as u8;
    let inner_pos = (position & 0xFF) as u8;
    (outer_bitmap_index, outer_pos, inner_pos)
}

/// Construct a 64-bit Position from (outer_bitmap_index, outer_pos, inner_pos)
pub const fn position_from_parts(outer_bitmap_index: u64, outer_pos: u8, inner_pos: u8) -> u64 {
    (outer_bitmap_index << 16) | ((outer_pos as u64) << 8) | (inner_pos as u64)
}

/// Extract ticks and column from a 64-bit Position
pub const fn ticks_from_position(position: u64) -> u64 {
    position >> 3
}

/// Extract column (0..7) from a 64-bit Position
pub const fn column_from_position(position: u64) -> u8 {
    (position & 0x07) as u8
}

/// Construct a 64-bit Position from ticks and column
pub const fn position_from_ticks(ticks: u64, column: u8) -> u64 {
    (ticks << 3) | ((column as u64) & 0x07)
}

/// Hierarchical group of make orders inside an inner bitmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerBitmapGroup {
    pub outer_pos: u8,
    pub updates: Vec<(u8, MakeAction)>, // (inner_pos, action)
}

/// Hierarchical group of inner bitmaps inside an outer bitmap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OuterBitmapGroup {
    pub outer_bitmap_index: u64,
    pub inner_bitmaps: Vec<InnerBitmapGroup>,
}

/// Group a list of positioned make orders into the 2-level bitmap hierarchy expected by goblin-core-v1.
/// Preserves order of operations while grouping by (outer_bitmap_index, outer_pos).
pub fn group_makes_by_bitmaps(
    makes: &[PositionedMakeOrder],
) -> Result<Vec<OuterBitmapGroup>, GoblinSdkError> {
    if makes.is_empty() {
        return Ok(Vec::new());
    }

    // Use BTreeMap to group deterministically by outer_bitmap_index -> outer_pos -> updates
    let mut outer_map: BTreeMap<u64, BTreeMap<u8, Vec<(u8, MakeAction)>>> = BTreeMap::new();

    for order in makes {
        let (outer_idx, outer_pos, inner_pos) = parts_from_position(order.position);
        outer_map
            .entry(outer_idx)
            .or_default()
            .entry(outer_pos)
            .or_default()
            .push((inner_pos, order.action));
    }

    if outer_map.len() > 3 {
        return Err(GoblinSdkError::OuterBitmapLimitExceeded(outer_map.len()));
    }

    let mut result = Vec::with_capacity(outer_map.len());
    for (outer_bitmap_index, inner_map) in outer_map {
        if inner_map.len() > 255 {
            return Err(GoblinSdkError::InnerBitmapLimitExceeded(inner_map.len()));
        }

        let mut inner_bitmaps = Vec::with_capacity(inner_map.len());
        for (outer_pos, updates) in inner_map {
            if updates.len() > 255 {
                return Err(GoblinSdkError::MakeUpdateLimitExceeded(updates.len()));
            }
            inner_bitmaps.push(InnerBitmapGroup { outer_pos, updates });
        }

        result.push(OuterBitmapGroup {
            outer_bitmap_index,
            inner_bitmaps,
        });
    }

    Ok(result)
}
