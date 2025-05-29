use crate::quantities::Delta;

#[repr(C, packed)]
pub struct IndexedTokenDelta {
    pub index: u8,
    pub delta: Delta,
}
