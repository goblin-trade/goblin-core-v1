use crate::{
    axis::token::token_list::{
        custom_erc20::CustomERC20Deltas, eth::ETHDelta, hardcoded_erc20::HardcodedERC20Deltas,
    },
    settlement::{global_delta::GlobalSender, ConstZero},
};

impl ConstZero for GlobalSender {
    const ZEROED: Self = Self::new(
        ETHDelta::ZEROED,
        HardcodedERC20Deltas::ZEROED,
        CustomERC20Deltas::ZEROED,
    );
}
