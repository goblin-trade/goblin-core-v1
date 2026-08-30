use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::{MarketHeader, OuterBitmapHeader},
    Ctx,
};

impl MarketHeader {
    pub fn execute_makes<MS: MarketSpec>(
        &self,
        reader: &ArgsReader,
        ctx: &mut Ctx<MS>,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            OuterBitmapHeader::process(reader, ctx)?;
        }

        Ok(())
    }
}
