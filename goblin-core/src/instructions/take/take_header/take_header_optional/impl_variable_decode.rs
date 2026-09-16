use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode, VariableDecode},
    instructions::{TakeFlags, TakeHeaderOptional},
    quantities::{FullPosU32, U32Variant},
    require,
};

impl<'a, In: LegMatcher> VariableDecode<'a> for TakeHeaderOptional<In> {
    type Flags = TakeFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.read_min_lots as usize + flags.read_limit as usize) * core::mem::size_of::<u32>()
    }

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self {
        let min_lots_to_fill_u32 = match flags.read_min_lots {
            true => U32Variant::<In::Lots>::raw_fixed_decode(reader),
            false => U32Variant::<In::Lots>::default(),
        };

        let limit_u32 = match flags.read_limit {
            true => FullPosU32::raw_fixed_decode(reader),
            false => In::DEFAULT_PRICE_LIMIT,
        };

        Self {
            min_lots_to_fill_u32,
            limit_u32,
        }
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.limit_u32 > FullPosU32::new(0),
            GoblinError::InvalidTakeArgs
        );
        Ok(())
    }
}
