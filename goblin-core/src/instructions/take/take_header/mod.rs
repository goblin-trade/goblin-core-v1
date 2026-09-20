pub mod take_header_main;
pub mod take_header_optional;

pub use take_header_main::*;
pub use take_header_optional::*;

use deku::{DekuError, DekuReader};

use crate::{
    axis::leg::LegMatcher,
    input_processor::ArgsReaderV2,
    quantities::{FullPosU32, U32Variant},
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
pub struct TakeHeader<In: LegMatcher> {
    /// The order size, i.e. number of lots to fill
    pub num_lots_u32: U32Variant<In::Lots>,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill_u32: U32Variant<In::Lots>,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit_u32: FullPosU32,
}

impl<In: LegMatcher> TakeHeader<In> {
    /// Decode the fixed-size main header followed by the flag-gated optional
    /// fields.
    pub fn decode<'a>(reader: &mut ArgsReaderV2<'a>) -> Result<Self, DekuError> {
        let main_header = TakeHeaderMain::<In>::from_reader_with_ctx(reader, ())?;
        let optional_header = TakeHeaderOptional::<In>::decode(reader, main_header.flags)?;

        Ok(Self {
            num_lots_u32: main_header.num_lots_u32,
            min_lots_to_fill_u32: optional_header.min_lots_to_fill_u32,
            limit_u32: optional_header.limit_u32,
        })
    }
}
