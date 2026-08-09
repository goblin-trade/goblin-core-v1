use crate::{settlement::ConstDefault, types::Tuple};

impl<T0, T1, K> ConstDefault for Tuple<T0, T1, K>
where
    T0: ConstDefault,
    T1: ConstDefault,
{
    const DEFAULT: Self = Tuple::new(T0::DEFAULT, T1::DEFAULT);
}
