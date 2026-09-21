use deku::{
    DekuError, DekuReader,
    ctx::{BitSize, Order},
    no_std_io::{Read, Seek},
    reader::Reader,
};

use crate::quantities::{Exp, Quantity, QuantityOps};

/// Decode a quantity from a bit field of the given width, delegating to the
/// inner ops.
///
/// This is what lets a bit-packed header declare a quantity field directly:
///
/// ```ignore
/// #[deku(bits = "30")]
/// pub num_lots_u32: U32Variant<In::Lots>,
/// ```
///
/// The raw bits are the quantity's inner value, so no `map`/re-wrapping or
/// `u64` widening is needed.
impl<'a, E, I> DekuReader<'a, (BitSize, Order)> for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps + DekuReader<'a, (BitSize, Order)>,
{
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        ctx: (BitSize, Order),
    ) -> Result<Self, DekuError> {
        let inner = I::from_reader_with_ctx(reader, ctx)?;
        Ok(Self::new(inner))
    }
}

#[cfg(feature = "encode")]
mod encode {
    use deku::{
        DekuError, DekuWriter,
        ctx::{BitSize, Order},
        no_std_io::{Seek, Write},
        writer::Writer,
    };

    use crate::quantities::{Exp, Quantity, QuantityOps};

    impl<E, I> DekuWriter<(BitSize, Order)> for Quantity<E, I>
    where
        E: Exp,
        I: QuantityOps + DekuWriter<(BitSize, Order)>,
    {
        #[inline]
        fn to_writer<W: Write + Seek>(
            &self,
            writer: &mut Writer<W>,
            ctx: (BitSize, Order),
        ) -> Result<(), DekuError> {
            self.inner.to_writer(writer, ctx)
        }
    }
}
