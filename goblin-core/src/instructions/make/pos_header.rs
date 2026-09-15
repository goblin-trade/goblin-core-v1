use crate::quantities::{BaseLots, PositionV2};

pub struct PosHeader {
    pub position: PositionV2,
    pub base_lots: BaseLots<u64>,
}
