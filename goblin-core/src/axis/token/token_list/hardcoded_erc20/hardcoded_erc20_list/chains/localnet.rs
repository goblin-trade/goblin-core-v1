use crate::axis::token::{
    token_list::hardcoded_erc20::HardcodedERC20List, token_marker::TokenData,
};
use hex_literal::hex;

pub const HARDCODED_ERC20_COUNT: usize = 2;

pub const HARDCODED_ERC20_LIST: HardcodedERC20List<HARDCODED_ERC20_COUNT> = HardcodedERC20List {
    inner: [
        TokenData {
            address: hex!("e1080224b632a93951a7cfa33eeea9fd81558b5e"),
            decimals: 18,
        },
        TokenData {
            address: hex!("3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e"),
            decimals: 18,
        },
    ],
};
