use crate::{
    quantities::{INNER_POS, OUTER_POS},
    state::bitmap::Bitmap,
};

pub type OuterBitmap = Bitmap<OUTER_POS>;
pub type InnerBitmap = Bitmap<INNER_POS>;
