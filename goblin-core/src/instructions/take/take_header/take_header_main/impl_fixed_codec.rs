use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, ArgsWriter, BitPack, FixedCodec},
    instructions::{TakeFlags, TakeHeaderMain},
    quantities::U32Variant,
    require,
};

impl<In: LegMatcher> FixedCodec for TakeHeaderMain<In> {
    const ENCODED_SIZE: usize = core::mem::size_of::<u32>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
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

    fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
        let raw_bytes = (self.flags.read_min_lots as u32)
            | ((self.flags.read_limit as u32) << 1)
            | ((self.num_lots_u32.to_raw() as u32) << 2);
        raw_bytes.raw_fixed_encode(writer);
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.num_lots_u32 > U32Variant::<In::Lots>::default(),
            GoblinError::InvalidTakeArgs
        );
        Ok(())
    }
}
