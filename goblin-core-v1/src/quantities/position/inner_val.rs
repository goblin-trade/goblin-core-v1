pub trait InnerVal: Sized + Into<u64> {
    fn from_u64(val: u64) -> Self;
}

impl InnerVal for u8 {
    fn from_u64(v: u64) -> Self {
        v as u8
    }
}

impl InnerVal for u64 {
    fn from_u64(v: u64) -> Self {
        v
    }
}
