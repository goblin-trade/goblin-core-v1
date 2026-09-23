use deku::ctx::{BitSize, Order};
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
use deku::{DekuError, DekuReader};

use super::HardcodedERC20Index;

/// Decode from a bit field of the width requested by the containing header.
///
/// The derived impl only covers the default `()` context; [`DekuBounds`] also
/// needs the bit-sized context so an index can be declared directly on a
/// bit-packed field (e.g. `#[deku(bits = "6")] token_index: TokenIndex`).
///
/// [`DekuBounds`]: crate::input_processor::DekuBounds
impl<'a> DekuReader<'a, (BitSize, Order)> for HardcodedERC20Index {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        (bit_size, order): (BitSize, Order),
    ) -> Result<Self, DekuError> {
        let inner = usize::from_reader_with_ctx(reader, (bit_size, order))?;
        if inner > Self::MAX_INNER {
            return Err(DekuError::Assertion(
                "HardcodedERC20Index inner exceeds MAX_INNER",
            ));
        }
        Ok(Self { inner })
    }
}
