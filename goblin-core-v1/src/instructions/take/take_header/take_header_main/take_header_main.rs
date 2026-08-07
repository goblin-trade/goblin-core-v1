use crate::{axis::leg::leg_matcher::LegMatcher, instructions::TakeFlags};

pub struct TakeHeaderMain<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// Flags indicating if optional take params should be decoded
    pub flags: TakeFlags,
}
