use crate::quantities::BaseLots;

pub enum UpdateType {
    Increase,
    Decrease,
}

// Use enum? This means adding a u8 extra field for each item or compressing
// it with BaseLots in a u64
pub struct UpdateHeader {
    pub base_lots: BaseLots,
}
