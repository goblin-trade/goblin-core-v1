use crate::{
    axis::update::{
        update_erc20::UpdateERC20, update_make::UpdateMake, Decrease, Increase, SameUpdatePair,
        UpdateETH, UpdateEnum, UpdateQuantity,
    },
    axis_helpers::AxisMarker,
    quantities::UnsidedLots,
    types::StoreReader,
};

pub trait UpdateMarker:
    UpdateMake
    + UpdateQuantity
    + UpdateERC20
    + UpdateETH
    + AxisMarker<Enum = UpdateEnum>
    + StoreReader<SameUpdatePair<UnsidedLots>, Result = UnsidedLots>
{
}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
