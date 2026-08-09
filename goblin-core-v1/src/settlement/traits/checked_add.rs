pub trait CheckedOps: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;

    fn checked_sub(self, rhs: Self) -> Option<Self>;
}
