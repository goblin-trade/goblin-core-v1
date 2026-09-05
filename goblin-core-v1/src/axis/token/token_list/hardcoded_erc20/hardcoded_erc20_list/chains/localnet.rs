use crate::{
    axis::{
        token::{
            token_list::hardcoded_erc20::HardcodedERC20List, token_marker::TokenData,
            HardcodedERC20,
        },
        HARDCODED_CALLER_LIST,
    },
    state::{SlotKey, StorePreimage},
};

pub const HARDCODED_ERC20_COUNT: usize = 2;

// TODO impl Index<HardcodedCallerIndex>
pub const HARDCODED_ERC20_LIST: HardcodedERC20List<HARDCODED_ERC20_COUNT> = HardcodedERC20List {
    inner: [
        TokenData {
            address: [
                0xe1, 0x08, 0x02, 0x24, 0xb6, 0x32, 0xa9, 0x39, 0x51, 0xa7, 0xcf, 0xa3, 0x3e, 0xee,
                0xa9, 0xfd, 0x81, 0x55, 0x8b, 0x5e,
            ],
            decimals: 18,
        },
        TokenData {
            address: [
                0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb,
                0x2a, 0xb1, 0x83, 0xeb, 0x2d, 0x0e,
            ],
            decimals: 18,
        },
    ],
};

// TODO impl Index<HardcodedCallerIndex>
pub const HARDCODED_ERC20_STORE_HASHES: [[SlotKey<StorePreimage<HardcodedERC20>>;
    HARDCODED_ERC20_COUNT];
    HARDCODED_CALLER_LIST.inner.len()] = [
    // Caller 0
    [
        StorePreimage {
            trader: HARDCODED_CALLER_LIST.inner[0],
            token_address: HARDCODED_ERC20_LIST.inner[0].address,
        }
        .const_hash(),
        StorePreimage {
            trader: HARDCODED_CALLER_LIST.inner[0],
            token_address: HARDCODED_ERC20_LIST.inner[1].address,
        }
        .const_hash(),
    ],
    // Caller 1
    [
        StorePreimage {
            trader: HARDCODED_CALLER_LIST.inner[1],
            token_address: HARDCODED_ERC20_LIST.inner[0].address,
        }
        .const_hash(),
        StorePreimage {
            trader: HARDCODED_CALLER_LIST.inner[1],
            token_address: HARDCODED_ERC20_LIST.inner[1].address,
        }
        .const_hash(),
    ],
];
