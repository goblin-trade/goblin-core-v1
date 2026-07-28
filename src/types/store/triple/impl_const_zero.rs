use crate::{settlement::ConstZero, types::Triple};

impl<T0, T1, T2, K> ConstZero for Triple<T0, T1, T2, K>
where
    T0: ConstZero,
    T1: ConstZero,
    T2: ConstZero,
{
    const ZEROED: Self = Triple::new(T0::ZEROED, T1::ZEROED, T2::ZEROED);
}
