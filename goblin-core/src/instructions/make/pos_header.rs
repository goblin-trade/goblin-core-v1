use crate::quantities::{BaseLots, FullPos};

pub struct PosHeader {
    pub position: FullPos,
    pub base_lots: BaseLots<u64>,
}
