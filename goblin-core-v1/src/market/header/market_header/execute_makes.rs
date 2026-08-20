use crate::{
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::{MarketHeader, OuterBitmapHeader, Readables, Writables},
};

impl<MS: MarketSpec> MarketHeader<MS> {
    pub fn execute_makes(
        &self,
        reader: &ArgsReader,
        readables: &Readables<MS>,
        writables: &mut Writables,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            OuterBitmapHeader::process(reader, readables, writables)?;
        }

        Ok(())
    }
}
