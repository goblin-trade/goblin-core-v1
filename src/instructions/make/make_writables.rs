use crate::{
    quantities::position::{INNER_POS, POS_1},
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
};

pub struct MakeWritables<'a> {
    pub local_delta: &'a mut LocalDelta,
    pub market_state: &'a mut MarketState,
    pub inner_bitmap_state: &'a mut Bitmap<POS_1, INNER_POS>,
}
