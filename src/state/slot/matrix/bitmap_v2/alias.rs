use crate::{
    quantities::{InnerPosV2, OuterPosV2},
    state::bitmap_v2::BitmapV2,
};

pub type OuterBitmapV2 = BitmapV2<OuterPosV2>;
pub type InnerBitmapV2 = BitmapV2<InnerPosV2>;
