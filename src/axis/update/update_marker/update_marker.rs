use crate::axis::update::{update_erc20::UpdateERC20, update_make::UpdateMake, Decrease, Increase};

pub trait UpdateMarker: UpdateMake + UpdateERC20 {}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
