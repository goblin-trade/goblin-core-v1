use deku::{DekuError, DekuReader};

use crate::{
    axis::leg::LegMatcher,
    input_processor::ArgsReaderV2,
    instructions::TakeFlags,
    quantities::{FullPosU32, U32Variant},
};

pub struct TakeHeaderOptional<In: LegMatcher> {
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill_u32: U32Variant<In::Lots>,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit_u32: FullPosU32,
}

impl<In: LegMatcher> TakeHeaderOptional<In> {
    /// Decode the optional fields gated by `flags`.
    ///
    /// `min_lots_to_fill_u32` is read when `flags.read_min_lots`; `limit_u32` is
    /// read when `flags.read_limit`. The skipped fields take their defaults.
    pub fn decode<'a>(reader: &mut ArgsReaderV2<'a>, flags: TakeFlags) -> Result<Self, DekuError> {
        let min_lots_to_fill_u32 = if flags.read_min_lots {
            U32Variant::<In::Lots>::from_reader_with_ctx(reader, ())?
        } else {
            U32Variant::<In::Lots>::default()
        };

        let limit_u32 = if flags.read_limit {
            FullPosU32::from_reader_with_ctx(reader, ())?
        } else {
            In::DEFAULT_PRICE_LIMIT
        };

        Ok(Self {
            min_lots_to_fill_u32,
            limit_u32,
        })
    }
}
