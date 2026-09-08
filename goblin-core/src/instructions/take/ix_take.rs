use crate::{
    axis::leg::LegMatcher,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, CompoundDecode},
    instructions::TakeHeader,
    matching::match_order,
    Ctx,
};

pub fn ix_take<MS: MarketSpec, In: LegMatcher>(
    reader: &ArgsReader,
    ctx: &mut Ctx<MS>,
) -> Result<(), GoblinError> {
    let header = TakeHeader::<In>::try_compound_decode(reader)?;
    match_order(header, ctx)
}
