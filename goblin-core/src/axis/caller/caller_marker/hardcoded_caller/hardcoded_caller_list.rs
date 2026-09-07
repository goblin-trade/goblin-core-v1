use crate::{axis::HardcodedCallerIndex, types::Address};
use hex_literal::hex;

pub struct HardcodedCallerList {
    pub inner: [Address; 2],
}

pub const HARDCODED_CALLER_LIST: HardcodedCallerList = HardcodedCallerList {
    inner: [
        [0u8; 20],
        hex!("3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e"),
    ],
};

impl HardcodedCallerList {
    pub fn index(address: &Address) -> Option<HardcodedCallerIndex> {
        HARDCODED_CALLER_LIST
            .inner
            .iter()
            .position(|&item| item == *address)
            .map(HardcodedCallerIndex::new)
    }
}
