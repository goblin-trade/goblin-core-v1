///! Type-level integers for exponents: -1, 0, +1

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct N1; // -1
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Z0; //  0
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct P1; // +1

pub trait Exp {}
impl Exp for N1 {}
impl Exp for Z0 {}
impl Exp for P1 {}
