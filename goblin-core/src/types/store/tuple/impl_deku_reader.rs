use deku::ctx::{BitSize, Order};
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
use deku::{DekuError, DekuReader};

use super::Tuple;

// `Tuple` is used as a byte-aligned wire field (e.g. `SamePair`, `TokenIndexPair`),
// so it must be readable by deku. The marker `K` is not read, which is why this is
// a hand-written impl: a `#[derive(DekuRead)]` would demand `K: DekuRead`.
impl<'a, T0, T1, K> DekuReader<'a, ()> for Tuple<T0, T1, K>
where
    T0: DekuReader<'a, ()>,
    T1: DekuReader<'a, ()>,
{
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let t0 = T0::from_reader_with_ctx(reader, ())?;
        let t1 = T1::from_reader_with_ctx(reader, ())?;
        Ok(Self::new(t0, t1))
    }
}

/// Read a bit-packed tuple whose per-component widths are supplied by the
/// field's `ctx` attribute, e.g.
/// `#[deku(bits = "2", ctx = "(1usize, 1usize)")]`.
impl<'a, T0, T1, K> DekuReader<'a, (BitSize, Order, (usize, usize))> for Tuple<T0, T1, K>
where
    T0: DekuReader<'a, (BitSize, Order)>,
    T1: DekuReader<'a, (BitSize, Order)>,
{
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        (_bit_size, order, (t0_width, t1_width)): (BitSize, Order, (usize, usize)),
    ) -> Result<Self, DekuError> {
        let t0 = T0::from_reader_with_ctx(reader, (BitSize(t0_width), order))?;
        let t1 = T1::from_reader_with_ctx(reader, (BitSize(t1_width), order))?;
        Ok(Self::new(t0, t1))
    }
}
