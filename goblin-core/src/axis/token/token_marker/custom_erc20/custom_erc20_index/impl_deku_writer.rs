use deku::DekuError;
#[cfg(feature = "encode")]
use deku::DekuWriter;
use deku::ctx::{BitSize, Order};
use deku::no_std_io::Seek;

#[cfg(feature = "encode")]
use crate::axis::token::token_marker::CustomERC20Index;

#[cfg(feature = "encode")]
impl DekuWriter<(BitSize, Order)> for CustomERC20Index {
    fn to_writer<W: deku::no_std_io::Write + Seek>(
        &self,
        writer: &mut deku::writer::Writer<W>,
        (bit_size, order): (BitSize, Order),
    ) -> Result<(), DekuError> {
        self.inner.to_writer(writer, (bit_size, order))
    }
}
