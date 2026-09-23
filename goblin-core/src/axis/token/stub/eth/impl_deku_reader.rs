use deku::ctx::{BitSize, Order};
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
use deku::{DekuError, DekuReader};

use super::ETHStub;

/// The derived impl only covers the default `()` context; [`DekuBounds`] also
/// needs the bit-sized context. The stub carries no data, so decoding reads
/// nothing regardless of the requested width.
///
/// [`DekuBounds`]: crate::input_processor::DekuBounds
impl<'a> DekuReader<'a, (BitSize, Order)> for ETHStub {
    fn from_reader_with_ctx<R: Read + Seek>(
        _reader: &mut Reader<R>,
        _ctx: (BitSize, Order),
    ) -> Result<Self, DekuError> {
        Ok(ETHStub)
    }
}
