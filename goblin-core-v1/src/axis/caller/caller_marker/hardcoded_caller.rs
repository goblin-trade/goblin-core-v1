use crate::{
    axis::{CallerMarker, HardcodedCaller},
    types::Address,
};

pub const HARDCODED_CALLER_COUNT: usize = 2;

pub const HARDCODED_CALLER_0: Address = [0u8; 20];
pub const HARDCODED_CALLER_1: Address = [
    0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb, 0x2a, 0xb1,
    0x83, 0xeb, 0x2d, 0x0e,
];

pub const HARDCODED_CALLERS: [Address; HARDCODED_CALLER_COUNT] =
    [HARDCODED_CALLER_0, HARDCODED_CALLER_1];

impl HardcodedCaller {
    pub fn get_caller_index(address: &Address) -> Option<usize> {
        if *address == HARDCODED_CALLER_0 {
            Some(0)
        } else if *address == HARDCODED_CALLER_1 {
            Some(1)
        } else {
            None
        }
    }
}

impl CallerMarker for HardcodedCaller {}
