use crate::{
    axis::leg::leg_matcher::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{CompoundDecode, DecodeCtx, FixedDecode, VariableDecode},
    instructions::{TakeHeader, TakeHeaderMain, TakeHeaderOptional},
};

impl<'a, In: LegMatcher> CompoundDecode<'a> for TakeHeader<In> {
    fn try_compound_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let main_header = TakeHeaderMain::<In>::try_fixed_decode(ctx)?;
        let optional_header =
            TakeHeaderOptional::<In>::raw_variable_decode(ctx, &main_header.flags);

        Ok(Self {
            num_lots: main_header.num_lots,
            min_lots_to_fill: optional_header.min_lots_to_fill,
            limit: optional_header.limit,
        })
    }
}
