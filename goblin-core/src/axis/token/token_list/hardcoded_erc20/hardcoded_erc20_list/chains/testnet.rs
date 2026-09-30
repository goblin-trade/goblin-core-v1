use crate::axis::token::{
    token_list::hardcoded_erc20::HardcodedERC20List, token_marker::TokenData,
};
use hex_literal::hex;

pub const HARDCODED_ERC20_COUNT: usize = 2;

pub const HARDCODED_ERC20_LIST: HardcodedERC20List<HARDCODED_ERC20_COUNT> = HardcodedERC20List {
    inner: [
        TokenData {
            address: hex!("11B57FE348584f042E436c6Bf7c3c3deF171de49"),
            decimals: 18,
        },
        TokenData {
            address: hex!("1294b86822ff4976BfE136cB06CF43eC7FCF2574"),
            decimals: 18,
        },
    ],
};
