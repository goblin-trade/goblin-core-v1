use crate::codec::{CodecResult, GoblinWrite, Writer};

use super::Tuple;

// Write the byte-aligned form, mirroring the `GoblinRead<'de, ()>` impl.
impl<T0, T1, K> GoblinWrite<()> for Tuple<T0, T1, K>
where
    T0: GoblinWrite<()>,
    T1: GoblinWrite<()>,
{
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.0.to_writer(writer, ())?;
        self.1.to_writer(writer, ())?;
        Ok(())
    }
}
