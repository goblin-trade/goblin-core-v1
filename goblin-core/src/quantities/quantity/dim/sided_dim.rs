use core::marker::PhantomData;

use crate::{
    axis::leg::LegMatcher,
    quantities::{Exp, UnsidedDim, add_exp::AddExp, sub_exp::SubExp},
};

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Ord)]
pub struct SidedDim<In: LegMatcher, L: Exp, U: Exp, A: Exp> {
    pub unsided: UnsidedDim<L, U, A>,
    _marker: PhantomData<In>,
}

impl<In: LegMatcher, L: Exp, U: Exp, A: Exp> From<UnsidedDim<L, U, A>> for SidedDim<In, L, U, A> {
    fn from(unsided: UnsidedDim<L, U, A>) -> Self {
        Self {
            unsided,
            _marker: PhantomData,
        }
    }
}

impl<In: LegMatcher, L: Exp, U: Exp, A: Exp> From<SidedDim<In, L, U, A>> for UnsidedDim<L, U, A> {
    fn from(value: SidedDim<In, L, U, A>) -> Self {
        value.unsided
    }
}

impl<In: LegMatcher, L: Exp, U: Exp, A: Exp> Exp for SidedDim<In, L, U, A> {}

/// Addition for SidedDim
impl<
    In: LegMatcher,
    L1: Exp + AddExp<L2>,
    U1: Exp + AddExp<U2>,
    A1: Exp + AddExp<A2>,
    L2: Exp,
    U2: Exp,
    A2: Exp,
> AddExp<SidedDim<In, L2, U2, A2>> for SidedDim<In, L1, U1, A1>
{
    type Output = SidedDim<
        In,
        <L1 as AddExp<L2>>::Output,
        <U1 as AddExp<U2>>::Output,
        <A1 as AddExp<A2>>::Output,
    >;
}

/// Subtraction for SidedDim
impl<
    In: LegMatcher,
    L1: Exp + SubExp<L2>,
    U1: Exp + SubExp<U2>,
    A1: Exp + SubExp<A2>,
    L2: Exp,
    U2: Exp,
    A2: Exp,
> SubExp<SidedDim<In, L2, U2, A2>> for SidedDim<In, L1, U1, A1>
{
    type Output = SidedDim<
        In,
        <L1 as SubExp<L2>>::Output,
        <U1 as SubExp<U2>>::Output,
        <A1 as SubExp<A2>>::Output,
    >;
}
