use deku::ctx::{BitSize, Order};
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

/// Write the bit-packed form, mirroring the
/// `DekuReader<'a, (BitSize, Order, (usize, usize))>` impl.
impl<T0, T1, K> DekuWriter<(BitSize, Order, (usize, usize))> for Tuple<T0, T1, K>
where
    T0: DekuWriter<(BitSize, Order)>,
    T1: DekuWriter<(BitSize, Order)>,
{
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        (_bit_size, order, (t0_width, t1_width)): (BitSize, Order, (usize, usize)),
    ) -> Result<(), DekuError> {
        self.0.to_writer(writer, (BitSize(t0_width), order))?;
        self.1.to_writer(writer, (BitSize(t1_width), order))?;
        Ok(())
    }
}
