pub mod token_delta;
pub mod token_settler;

pub use token_delta::*;
pub use token_settler::*;

use crate::{
    axis::token::{
        Token,
        token_list::{
            custom_erc20::CustomERC20Deltas, eth::ETHDelta, hardcoded_erc20::HardcodedERC20Deltas,
        },
    },
    types::Triple,
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;
