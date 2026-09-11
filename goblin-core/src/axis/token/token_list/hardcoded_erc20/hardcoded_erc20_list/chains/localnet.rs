use crate::axis::token::{
    token_list::hardcoded_erc20::HardcodedERC20List, token_marker::TokenData,
};
use hex_literal::hex;

pub const HARDCODED_ERC20_COUNT: usize = 2;

pub const HARDCODED_ERC20_LIST: HardcodedERC20List<HARDCODED_ERC20_COUNT> = HardcodedERC20List {
    inner: [
        TokenData {
            address: hex!("85D9a8a4bd77b9b5559c1B7FCb8eC9635922Ed49"),
            decimals: 18,
        },
        TokenData {
            address: hex!("4A2bA922052bA54e29c5417bC979Daaf7D5Fe4f4"),
            decimals: 18,
        },
    ],
};
