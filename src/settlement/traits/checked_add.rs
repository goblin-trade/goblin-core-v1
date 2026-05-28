pub trait CheckedAdd: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
}
