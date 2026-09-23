use deku::ctx::{BitSize, Order};
use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use super::HardcodedERC20Index;

impl DekuWriter<(BitSize, Order)> for HardcodedERC20Index {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        (bit_size, order): (BitSize, Order),
    ) -> Result<(), DekuError> {
        self.inner.to_writer(writer, (bit_size, order))
    }
}
