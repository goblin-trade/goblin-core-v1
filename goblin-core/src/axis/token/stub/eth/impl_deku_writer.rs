use deku::ctx::{BitSize, Order};
use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use super::ETHStub;

impl DekuWriter<(BitSize, Order)> for ETHStub {
    fn to_writer<W: Write + Seek>(
        &self,
        _writer: &mut Writer<W>,
        _ctx: (BitSize, Order),
    ) -> Result<(), DekuError> {
        Ok(())
    }
}
