use crate::{settlement::ConstZero, types::Tuple};

impl<T0, T1, K> ConstZero for Tuple<T0, T1, K>
where
    T0: ConstZero,
    T1: ConstZero,
{
    const ZEROED: Self = Tuple::new(T0::ZEROED, T1::ZEROED);
}
