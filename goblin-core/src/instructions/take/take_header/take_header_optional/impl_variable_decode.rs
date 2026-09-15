use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode, VariableDecode},
    instructions::{TakeFlags, TakeHeaderOptional},
    quantities::{FullPos, FullPosU32, FullPosition, ScaledPosition},
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
            true => FullPosU32::raw_fixed_decode(reader).scale_up(),
            false => In::DEFAULT_PRICE_LIMIT,
        };

        Self {
            min_lots_to_fill,
            limit,
        }
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(self.limit > FullPos::ZERO, GoblinError::InvalidTakeArgs);
        Ok(())
    }
}
