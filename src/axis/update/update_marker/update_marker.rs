use crate::{
    axis::update::{
        update_erc20::UpdateERC20, update_make::UpdateMake, Decrease, Increase, SameUpdatePair,
        UpdateETH,
    },
    quantities::UnsidedLots,
    types::StoreReader,
};

pub trait UpdateMarker:
    UpdateMake
    + UpdateERC20
    + UpdateETH
    + StoreReader<SameUpdatePair<UnsidedLots>, Result = UnsidedLots>
{
}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
