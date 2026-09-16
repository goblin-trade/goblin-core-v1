use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, CompoundDecode, FixedDecode, VariableDecode},
    instructions::{TakeHeader, TakeHeaderMain, TakeHeaderOptional},
};

impl<'a, In: LegMatcher> CompoundDecode<'a> for TakeHeader<In> {
    fn try_compound_decode(reader: &ArgsReader) -> Result<Self, GoblinError> {
        let main_header = TakeHeaderMain::<In>::try_fixed_decode(reader)?;
        let optional_header =
            TakeHeaderOptional::<In>::raw_variable_decode(reader, &main_header.flags);

        Ok(Self {
            num_lots_u32: main_header.num_lots_u32,
            min_lots_to_fill_u32: optional_header.min_lots_to_fill_u32,
            limit_u32: optional_header.limit_u32,
        })
    }
}
