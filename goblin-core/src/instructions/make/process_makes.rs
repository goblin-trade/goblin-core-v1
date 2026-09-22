use crate::{
    Ctx,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::{MarketHeader, OuterBitmapHeader},
};

pub fn process_makes<MS: MarketSpec>(
    header: &MarketHeader,
    reader: &mut ArgsReader<'_>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    for _ in 0..header.outer_bitmap_count {
        OuterBitmapHeader::process(reader, ctx)?;
    }

    Ok(())
}
