use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_spec::MarketSpec, Readables, Writables},
    },
    goblin_error::GoblinError,
    input_processor::DecodeCtx,
    instructions::take::take_header::TakeHeader,
    matching::match_order,
};

pub fn ix_take<MS: MarketSpec, In: LegMatcher>(
    ctx: &DecodeCtx,
    readables: &Readables<MS>,
    writables: &mut Writables,
) -> Result<(), GoblinError> {
    let header = TakeHeader::<In>::try_decode(ctx)?;
    match_order(header, readables, writables)
}
