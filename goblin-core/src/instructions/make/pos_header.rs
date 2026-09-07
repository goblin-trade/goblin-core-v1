use crate::quantities::{BaseLots, Position};

pub struct PosHeader {
    pub position: Position,
    pub base_lots: BaseLots,
}
