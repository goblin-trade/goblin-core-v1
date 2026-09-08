use crate::{
    quantities::{OUTER_POS, POS_0},
    state::{SlotState, bitmap::Bitmap},
};

impl<const BITS: u16, const INNER_BITS: u16> SlotState for Bitmap<BITS, INNER_BITS> {}
const _: () = <Bitmap<POS_0, OUTER_POS> as SlotState>::_ASSERT;
