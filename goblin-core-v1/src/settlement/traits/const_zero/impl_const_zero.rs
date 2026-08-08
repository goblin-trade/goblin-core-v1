use core::marker::PhantomData;

use crate::settlement::ConstZero;

impl ConstZero for u64 {
    const ZEROED: Self = 0;
}

impl ConstZero for i64 {
    const ZEROED: Self = 0;
}

impl ConstZero for usize {
    const ZEROED: Self = 0;
}

impl ConstZero for u8 {
    const ZEROED: Self = 0;
}

impl<T> ConstZero for PhantomData<T> {
    const ZEROED: Self = PhantomData;
}
