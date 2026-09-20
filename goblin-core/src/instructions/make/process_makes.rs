use crate::{
    Ctx,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::ArgsReaderV2,
    market::{MarketHeader, OuterBitmapHeader},
};

pub fn process_makes<MS: MarketSpec>(
    header: &MarketHeader,
    reader: &mut ArgsReaderV2<'_>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    for _ in 0..header.outer_bitmap_count {
        OuterBitmapHeader::process(reader, ctx)?;
    }

    Ok(())
}
