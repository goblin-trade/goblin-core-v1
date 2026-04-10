use crate::{
    quantities::{INNER_POS_V2, OUTER_POS_V2},
    state::bitmap_v2::BitmapV2,
};

pub type OuterBitmapV2 = BitmapV2<OUTER_POS_V2>;
pub type InnerBitmapV2 = BitmapV2<INNER_POS_V2>;
