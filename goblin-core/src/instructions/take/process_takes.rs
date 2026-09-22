use crate::{
    Ctx, axis_helpers::MarketSpec, for_axes, goblin_error::GoblinError,
    input_processor::ArgsReader, instructions::ix_take, market::MarketHeader, types::StoreReader,
};

pub fn process_takes<MS: MarketSpec>(
    header: &MarketHeader,
    reader: &mut ArgsReader<'_>,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    for_axes!(In => {
        if In::get(&header.execute_takes) {
            ix_take::<MS, In>(reader, ctx)?;
        }
    });

    Ok(())
}
