pub mod take_header_main;
pub mod take_header_optional;

pub use take_header_main::*;
pub use take_header_optional::*;

mod impl_compound_decode;

use crate::{axis::leg::LegMatcher, quantities::Position};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakeHeader<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots: In::Lots,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill: In::Lots,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit: Position,
}
