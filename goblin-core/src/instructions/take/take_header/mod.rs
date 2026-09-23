pub mod take_flags;

pub use take_flags::*;

#[cfg(feature = "encode")]
use deku::DekuWriter;
#[cfg(feature = "encode")]
use deku::no_std_io::Write;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
#[cfg(feature = "encode")]
use deku::writer::Writer;
use deku::{DekuError, DekuReader};

#[cfg(feature = "encode")]
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::{
    axis::leg::LegMatcher,
    quantities::{FullPosU32, QuantityOps, U32Variant},
};

/// Instructions for a limit order. Limit orders are also known as market orders or immediate or cancel (IOC).
///
/// Fill or Kill (FoK) is a special case of limit orders where the entire amount must be filled
/// otherwise the order gets cancelled, i.e.
///
/// num_lots == min_lots_to_fill
///
/// The wire layout is a bit-packed `u32` (flags + `num_lots_u32`) followed by
/// the flag-gated optional fields. deku's `bits` feature is off, so the leading
/// `u32` is decoded as an LSB-first lane and the optional fields stay byte
/// aligned with the default (unit) context.
pub struct TakeHeader<In: LegMatcher> {
    /// Flags indicating if optional take params should be decoded
    pub flags: TakeFlags,

    /// The order size, i.e. number of lots to fill
    pub num_lots_u32: U32Variant<In::Lots>,

    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    /// Read only when `flags.read_min_lots` is set, otherwise defaults to zero.
    pub min_lots_to_fill_u32: U32Variant<In::Lots>,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    /// Read only when `flags.read_limit` is set, otherwise defaults to
    /// [`In::DEFAULT_PRICE_LIMIT`](crate::axis::leg::LegConstants::DEFAULT_PRICE_LIMIT).
    pub limit_u32: FullPosU32,
}

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

#[cfg(feature = "encode")]
impl<In: LegMatcher> DekuWriter<()> for TakeHeader<In> {
    #[inline]
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.flags.read_min_lots, 0);
        lane = pack_bool(lane, self.flags.read_limit, 1);
        lane = pack(lane, self.num_lots_u32.to_raw(), 2, 30);
        write_lane::<W, 4>(writer, lane)?;

        if self.flags.read_min_lots {
            self.min_lots_to_fill_u32.to_writer(writer, ())?;
        }
        if self.flags.read_limit {
            self.limit_u32.to_writer(writer, ())?;
        }

        Ok(())
    }
}
