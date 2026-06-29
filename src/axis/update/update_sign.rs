use crate::axis::update::{Decrease, Increase};

pub trait UpdateSign: Sized {
    const SIGN: i64;
}
impl UpdateSign for Decrease {
    const SIGN: i64 = -1;
}

impl UpdateSign for Increase {
    const SIGN: i64 = 1;
}
