pub mod take_flags;

pub use take_flags::*;

mod impl_fixed_codec;

use crate::{axis::leg::LegMatcher, quantities::U32Variant};

pub struct TakeHeaderMain<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots_u32: U32Variant<In::Lots>,

    /// Flags indicating if optional take params should be decoded
    pub flags: TakeFlags,
}
