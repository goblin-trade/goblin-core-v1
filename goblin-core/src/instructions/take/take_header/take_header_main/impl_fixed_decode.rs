use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    instructions::{TakeFlags, TakeHeaderMain},
    quantities::U32Variant,
    require,
};

impl<'a, In: LegMatcher> FixedDecode<'a> for TakeHeaderMain<In> {
    const ENCODED_SIZE: usize = core::mem::size_of::<u32>();

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let raw_bytes = u32::raw_fixed_decode(reader);

        let read_min_lots = raw_bytes & 0b01 != 0;
        let read_limit = raw_bytes & 0b10 != 0;
        let num_lots = U32Variant::<In::Lots>::from(raw_bytes >> 2);

        Self {
            num_lots_u32: num_lots,
            flags: TakeFlags {
                read_min_lots,
                read_limit,
            },
        }
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.num_lots_u32 > U32Variant::<In::Lots>::default(),
            GoblinError::InvalidTakeArgs
        );
        Ok(())
    }
}
