pub mod sided_dim;

pub use sided_dim::*;

use crate::quantities::{Exp, add_exp::AddExp, sub_exp::SubExp};

use core::marker::PhantomData;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Dim<Base: Exp, Quote: Exp, T: Exp>(PhantomData<(Base, Quote, T)>);

impl<Base: Exp, Quote: Exp, T: Exp> Exp for Dim<Base, Quote, T> {}

//
// Addition for Dim
//
impl<
    Base1: Exp + AddExp<Base2>,
    Quote1: Exp + AddExp<Quote2>,
    T1: Exp + AddExp<T2>,
    Base2: Exp,
    Quote2: Exp,
    T2: Exp,
> AddExp<Dim<Base2, Quote2, T2>> for Dim<Base1, Quote1, T1>
{
    type Output = Dim<
        <Base1 as AddExp<Base2>>::Output,
        <Quote1 as AddExp<Quote2>>::Output,
        <T1 as AddExp<T2>>::Output,
    >;
}

/// Subtraction for Dim
impl<
    Base1: Exp + SubExp<Base2>,
    Quote1: Exp + SubExp<Quote2>,
    T1: Exp + SubExp<T2>,
    Base2: Exp,
    Quote2: Exp,
    T2: Exp,
> SubExp<Dim<Base2, Quote2, T2>> for Dim<Base1, Quote1, T1>
{
    type Output = Dim<
        <Base1 as SubExp<Base2>>::Output,
        <Quote1 as SubExp<Quote2>>::Output,
        <T1 as SubExp<T2>>::Output,
    >;
}
