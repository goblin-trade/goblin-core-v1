use crate::{settlement::ConstDefault, types::Triple};

impl<T0, T1, T2, K> ConstDefault for Triple<T0, T1, T2, K>
where
    T0: ConstDefault,
    T1: ConstDefault,
    T2: ConstDefault,
{
    const DEFAULT: Self = Triple::new(T0::DEFAULT, T1::DEFAULT, T2::DEFAULT);
}
