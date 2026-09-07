use crate::{axis::HardcodedCallerIndex, types::Address};

pub struct HardcodedCallerList {
    pub inner: [Address; 2],
}

pub const HARDCODED_CALLER_LIST: HardcodedCallerList = HardcodedCallerList {
    // inner: [],
    inner: [
        [0u8; 20],
        [
            0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb,
            0x2a, 0xb1, 0x83, 0xeb, 0x2d, 0x0e,
        ],
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
