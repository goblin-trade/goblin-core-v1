pub mod take_flags;

pub use take_flags::*;

use goblin_macros::fixed_codec;

use crate::{axis::leg::LegMatcher, quantities::U32Variant};

/// Take header, packed into one `u32`: the two flags occupy the low bits and
/// `num_lots_u32` occupies the remaining 30 bits.
///
/// The `num_lots_u32 > 0` check lives at the call site (`ix_take`) rather than in
/// a `validate` hook, since it is already enforced there.
#[fixed_codec(bits = 32)]
pub struct TakeHeaderMain<In: LegMatcher> {
    /// Flags indicating if optional take params should be decoded
    pub flags: TakeFlags,

    /// The order size, i.e. number of lots to fill
    pub num_lots_u32: U32Variant<In::Lots>,
}
