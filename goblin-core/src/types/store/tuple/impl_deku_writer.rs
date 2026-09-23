use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;
use deku::{DekuError, DekuWriter};

use super::Tuple;

// Write the byte-aligned form, mirroring the `DekuReader<'a, ()>` impl.
impl<T0, T1, K> DekuWriter<()> for Tuple<T0, T1, K>
where
    T0: DekuWriter<()>,
    T1: DekuWriter<()>,
{
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        self.0.to_writer(writer, ())?;
        self.1.to_writer(writer, ())?;
        Ok(())
    }
}
