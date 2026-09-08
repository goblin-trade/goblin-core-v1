pub mod take_flags;

pub use take_flags::*;

mod impl_fixed_decode;

use crate::axis::leg::LegMatcher;

pub struct TakeHeaderMain<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// Flags indicating if optional take params should be decoded
    pub flags: TakeFlags,
}
