use deku::DekuReader;

use crate::{
    Ctx, axis::leg::LegMatcher, axis_helpers::MarketSpec, goblin_error::GoblinError,
    input_processor::ArgsReaderV2, instructions::TakeHeader, matching::match_order,
};

pub fn ix_take<MS: MarketSpec, In: LegMatcher>(
    reader: &mut ArgsReaderV2<'_>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let header = TakeHeader::<In>::from_reader_with_ctx(reader, ())
        .map_err(|_| GoblinError::InvalidPayload)?;
    match_order(header, ctx)
}
