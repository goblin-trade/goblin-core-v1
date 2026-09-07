use crate::{
    axis::token::{
        token_list::{
            custom_erc20::CustomERC20Deltas, eth::ETHDelta, hardcoded_erc20::HardcodedERC20Deltas,
        },
        Token,
    },
    types::Triple,
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;
