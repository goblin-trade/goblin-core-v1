use core::marker::PhantomData;

use crate::settlement::ConstDefault;

impl ConstDefault for u64 {
    const DEFAULT: Self = 0;
}

impl ConstDefault for i64 {
    const DEFAULT: Self = 0;
}

impl ConstDefault for usize {
    const DEFAULT: Self = 0;
}

impl ConstDefault for u8 {
    const DEFAULT: Self = 0;
}

impl<T> ConstDefault for PhantomData<T> {
    const DEFAULT: Self = PhantomData;
}
