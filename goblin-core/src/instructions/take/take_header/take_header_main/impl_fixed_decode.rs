use crate::{
    axis::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    instructions::{TakeFlags, TakeHeaderMain},
    require,
};

impl<'a, In: LegMatcher> FixedDecode<'a> for TakeHeaderMain<In> {
    const ENCODED_SIZE: usize = core::mem::size_of::<u64>();

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let raw_bytes = u64::raw_fixed_decode(reader);

        let read_min_lots = raw_bytes & 0b01 != 0;
        let read_limit = raw_bytes & 0b10 != 0;
        let num_lots = In::Lots::from(raw_bytes >> 2);

        Self {
            num_lots,
            flags: TakeFlags {
                read_min_lots,
                read_limit,
            },
        }
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.num_lots > In::Lots::default(),
            GoblinError::InvalidTakeArgs
        );
        Ok(())
    }
}
