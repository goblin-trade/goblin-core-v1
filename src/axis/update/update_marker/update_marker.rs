use crate::axis::update::{
    update_erc20::UpdateERC20, update_make::UpdateMake, Decrease, Increase, UpdateETH,
};

pub trait UpdateMarker: UpdateMake + UpdateERC20 + UpdateETH {}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
