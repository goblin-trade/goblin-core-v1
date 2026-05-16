use crate::{
    quantities::{INNER_POS, OUTER_POS, POS_0, POS_1},
    state::bitmap::Bitmap,
};

pub type OuterBitmap = Bitmap<POS_0, OUTER_POS>;
pub type InnerBitmap = Bitmap<POS_1, INNER_POS>;
