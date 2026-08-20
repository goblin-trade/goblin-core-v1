use crate::{
    axis::leg::leg_matcher::LegMatcher,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, CompoundDecode},
    instructions::take::take_header::TakeHeader,
    market::{Readables, Writables},
    matching::match_order,
};

pub fn ix_take<MS: MarketSpec, In: LegMatcher>(
    reader: &ArgsReader,
    readables: &Readables<MS>,
    writables: &mut Writables,
) -> Result<(), GoblinError> {
    let header = TakeHeader::<In>::try_compound_decode(reader)?;
    match_order(header, readables, writables)
}
