use crate::types::Pair;

pub struct MarketHeader {
    /// Whether to read decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute base-in and quote-in take orders
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,
}
