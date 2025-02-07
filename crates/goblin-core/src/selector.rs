// Constants for function selectors
// Find hash from https://emn178.github.io/online-tools/keccak_256.html
pub const SET_COUNT_SELECTOR: [u8; 4] = [0xd1, 0x4e, 0x62, 0xb8]; // keccak256("setCount(uint256)")[:4] = d14e62b8
pub const GET_COUNT_SELECTOR: [u8; 4] = [0xa8, 0x7d, 0x94, 0x2c]; // keccak256("getCount()")[:4] = a87d942c
