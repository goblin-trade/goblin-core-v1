use crate::{
    axis::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode, VariableDecode},
    instructions::{TakeFlags, TakeHeaderOptional},
    quantities::Position,
    require,
};

impl<'a, In: LegMatcher> VariableDecode<'a> for TakeHeaderOptional<In> {
    type Flags = TakeFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.read_min_lots as usize + flags.read_min_lots as usize) * core::mem::size_of::<u64>()
    }

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self {
        let min_lots_to_fill = match flags.read_min_lots {
            true => In::Lots::raw_fixed_decode(reader),
            false => In::Lots::default(),
        };

        let limit = match flags.read_limit {
            true => Position::raw_fixed_decode(reader),
            false => In::DEFAULT_PRICE_LIMIT,
        };

        Self {
            min_lots_to_fill,
            limit,
        }
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(self.limit > Position::ZERO, GoblinError::InvalidTakeArgs);
        Ok(())
    }
}
