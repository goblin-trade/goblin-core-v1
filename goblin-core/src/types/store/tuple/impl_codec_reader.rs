use crate::codec::{CodecResult, GoblinRead, Reader};

use super::Tuple;

// `Tuple` is used as a byte-aligned wire field (e.g. `SamePair`, `TokenIndexPair`),
// so it must be readable by the codec. The marker `K` is not read, which is why this is
// a hand-written impl: a derive would demand `K: GoblinRead`.
impl<'de, T0, T1, K> GoblinRead<'de, ()> for Tuple<T0, T1, K>
where
    T0: GoblinRead<'de, ()>,
    T1: GoblinRead<'de, ()>,
{
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let t0 = T0::from_reader_with_ctx(reader, ())?;
        let t1 = T1::from_reader_with_ctx(reader, ())?;
        Ok(Self::new(t0, t1))
    }
}
