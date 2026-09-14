use core::marker::PhantomData;

use crate::{
    axis::leg::LegMatcher,
    quantities::{Exp, UnsidedDim, add_exp::AddExp, sub_exp::SubExp},
};

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Ord)]
pub struct SidedDim<L: Exp, U: Exp, A: Exp, In: LegMatcher> {
    pub unsided: UnsidedDim<L, U, A>,
    _marker: PhantomData<In>,
}

impl<L: Exp, U: Exp, A: Exp, In: LegMatcher> From<UnsidedDim<L, U, A>> for SidedDim<L, U, A, In> {
    fn from(unsided: UnsidedDim<L, U, A>) -> Self {
        Self {
            unsided,
            _marker: PhantomData,
        }
    }
}

impl<L: Exp, U: Exp, A: Exp, In: LegMatcher> From<SidedDim<L, U, A, In>> for UnsidedDim<L, U, A> {
    fn from(value: SidedDim<L, U, A, In>) -> Self {
        value.unsided
    }
}

impl<L: Exp, U: Exp, A: Exp, In: LegMatcher> Exp for SidedDim<L, U, A, In> {}

/// Addition for SidedDim
impl<
    L1: Exp + AddExp<L2>,
    U1: Exp + AddExp<U2>,
    A1: Exp + AddExp<A2>,
    L2: Exp,
    U2: Exp,
    A2: Exp,
    In: LegMatcher,
> AddExp<SidedDim<L2, U2, A2, In>> for SidedDim<L1, U1, A1, In>
{
    type Output = SidedDim<
        <L1 as AddExp<L2>>::Output,
        <U1 as AddExp<U2>>::Output,
        <A1 as AddExp<A2>>::Output,
        In,
    >;
}

/// Subtraction for SidedDim
impl<
    L1: Exp + SubExp<L2>,
    U1: Exp + SubExp<U2>,
    A1: Exp + SubExp<A2>,
    L2: Exp,
    U2: Exp,
    A2: Exp,
    In: LegMatcher,
> SubExp<SidedDim<L2, U2, A2, In>> for SidedDim<L1, U1, A1, In>
{
    type Output = SidedDim<
        <L1 as SubExp<L2>>::Output,
        <U1 as SubExp<U2>>::Output,
        <A1 as SubExp<A2>>::Output,
        In,
    >;
}
