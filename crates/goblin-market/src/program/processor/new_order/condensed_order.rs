use stylus_sdk::alloy_primitives::FixedBytes;

use crate::quantities::{BaseLots, Ticks, WrapperU64};

pub struct CondensedOrder {
    // Order price in ticks
    pub price_in_ticks: Ticks,

    // Order size
    pub size_in_base_lots: BaseLots,

    // Whether to track block or unix timestamp
    pub track_block: bool,

    // The last valid block or unix timestamp, depending on the value of
    // track_block. Set value as 0 to disable FOK.
    pub last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl From<&FixedBytes<21>> for CondensedOrder {
    fn from(bytes: &FixedBytes<21>) -> Self {
        CondensedOrder {
            price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
            size_in_base_lots: BaseLots::new(u64::from_be_bytes(bytes[8..16].try_into().unwrap())),
            track_block: (bytes[16] & 0b0000_0001) != 0,
            last_valid_block_or_unix_timestamp_in_seconds: u32::from_be_bytes(
                bytes[17..21].try_into().unwrap(),
            ),
        }
    }
}
