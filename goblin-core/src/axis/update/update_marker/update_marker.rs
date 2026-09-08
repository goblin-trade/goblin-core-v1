use crate::{
    axis::update::{
        Decrease, Increase, SameUpdatePair, UpdateAddress, UpdateETH, UpdateEnum, UpdateQuantity,
        update_erc20::UpdateERC20, update_make::UpdateMake,
    },
    axis_helpers::AxisMarker,
    quantities::UnsidedLots,
    types::StoreReader,
};

pub trait UpdateMarker:
    UpdateMake
    + UpdateQuantity
    + UpdateAddress
    + UpdateERC20
    + UpdateETH
    + AxisMarker<Enum = UpdateEnum>
    + StoreReader<SameUpdatePair<UnsidedLots>, Result = UnsidedLots>
{
}

impl UpdateMarker for Increase {}

impl UpdateMarker for Decrease {}
