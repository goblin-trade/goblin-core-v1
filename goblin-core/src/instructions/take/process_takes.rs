use crate::{
    axis_helpers::MarketSpec, for_axes, goblin_error::GoblinError, input_processor::ArgsReader,
    instructions::ix_take, market::MarketHeader, types::StoreReader, Ctx,
};

pub fn process_takes<MS: MarketSpec>(
    header: &MarketHeader,
    reader: &ArgsReader,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    for_axes!(In => {
        if In::get(&header.execute_takes) {
            ix_take::<MS, In>(reader, ctx)?;
        }
    });

    Ok(())
}
