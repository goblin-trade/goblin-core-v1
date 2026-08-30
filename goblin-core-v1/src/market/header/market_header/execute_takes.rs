use super::MarketHeader;
use crate::{
    axis_helpers::MarketSpec, for_axes, goblin_error::GoblinError, input_processor::ArgsReader,
    instructions::ix_take, types::StoreReader, Ctx,
};

impl MarketHeader {
    pub fn execute_takes<MS: MarketSpec>(
        &self,
        reader: &ArgsReader,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        for_axes!(In => {
            if In::get(&self.execute_takes) {
                ix_take::<MS, In>(reader, ctx)?;
            }
        });

        Ok(())
    }
}
