pub mod take_flags;

pub use take_flags::*;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{
    axis::leg::LegMatcher,
    quantities::{FullPosU32, U32Variant},
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
///
/// The wire layout is a bit-packed `u32` (flags + `num_lots_u32`) followed by
/// the flag-gated optional fields. The bit order is declared per-field rather
/// than at the top level so the byte-aligned optional fields keep the default
/// (unit) context.
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct TakeHeader<In: LegMatcher> {
    /// Flags indicating if optional take params should be decoded
    #[deku(bit_order = "lsb")]
    pub flags: TakeFlags,

    /// The order size, i.e. number of lots to fill
    #[deku(bits = "30", bit_order = "lsb")]
    pub num_lots_u32: U32Variant<In::Lots>,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    /// Read only when `flags.read_min_lots` is set, otherwise defaults to zero.
    #[deku(cond = "flags.read_min_lots")]
    pub min_lots_to_fill_u32: U32Variant<In::Lots>,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    /// Read only when `flags.read_limit` is set, otherwise defaults to
    /// [`In::DEFAULT_PRICE_LIMIT`](crate::axis::leg::LegConstants::DEFAULT_PRICE_LIMIT).
    #[deku(cond = "flags.read_limit", default = "In::DEFAULT_PRICE_LIMIT")]
    pub limit_u32: FullPosU32,
}
