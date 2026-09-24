use deku::{
    DekuError, DekuReader,
    no_std_io::{Read, Seek},
    reader::Reader,
};

use crate::{
    axis::leg::LegMatcher,
    input_processor::bit_lane::{read_lane, unpack, unpack_bool},
    instructions::{TakeFlags, TakeHeader},
    quantities::{FullPosU32, QuantityOps, U32Variant},
};

impl<'a, In: LegMatcher> DekuReader<'a, ()> for TakeHeader<In> {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 4>(reader)?;

        let flags = TakeFlags {
            read_min_lots: unpack_bool(lane, 0),
            read_limit: unpack_bool(lane, 1),
        };
        let num_lots_u32 = U32Variant::<In::Lots>::from_raw(unpack(lane, 2, 30));

        let min_lots_to_fill_u32 = if flags.read_min_lots {
            U32Variant::<In::Lots>::from_reader_with_ctx(reader, ())?
        } else {
            U32Variant::<In::Lots>::default()
        };
        let limit_u32 = if flags.read_limit {
            FullPosU32::from_reader_with_ctx(reader, ())?
        } else {
            In::DEFAULT_PRICE_LIMIT
        };

        Ok(Self {
            flags,
            num_lots_u32,
            min_lots_to_fill_u32,
            limit_u32,
        })
    }
}
