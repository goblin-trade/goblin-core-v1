use crate::{
    axis::update::{
        update_erc20::UpdateERC20, update_make::UpdateMake, update_sign::UpdateSign, Decrease,
        Increase, SameUpdatePair, UpdateETH, UpdateEnum,
    },
    axis_helpers::AxisMarker,
    quantities::UnsidedLots,
    types::StoreReader,
};

pub trait UpdateMarker:
    UpdateMake
    + UpdateSign
    + UpdateERC20
    + UpdateETH
    + AxisMarker<Enum = UpdateEnum>
    + StoreReader<SameUpdatePair<UnsidedLots>, Result = UnsidedLots>
{
}

impl UpdateMarker for Increase {}
impl UpdateMarker for Decrease {}
