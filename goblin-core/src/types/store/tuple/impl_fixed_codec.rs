use crate::{
    input_processor::{ArgsReader, ArgsWriter, FixedCodec},
    types::Tuple,
};

impl<T0, T1, K> FixedCodec for Tuple<T0, T1, K>
where
    T0: FixedCodec,
    T1: FixedCodec,
{
    const ENCODED_SIZE: usize = T0::ENCODED_SIZE + T1::ENCODED_SIZE;

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        Self::new(T0::raw_fixed_decode(reader), T1::raw_fixed_decode(reader))
    }

    fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
        T0::raw_fixed_encode(&self.0, writer);
        T1::raw_fixed_encode(&self.1, writer);
    }
}
