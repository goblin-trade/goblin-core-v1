///! Compact sided dimension (L, U, A)
use core::marker::PhantomData;

use crate::quantities::{add_exp::AddExp, sub_exp::SubExp, Exp};

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SidedDim<L: Exp, U: Exp, A: Exp>(PhantomData<(L, U, A)>);

impl<L: Exp, U: Exp, A: Exp> Exp for SidedDim<L, U, A> {}

/// Addition for SidedDim
impl<
        L1: Exp + AddExp<L2>,
        U1: Exp + AddExp<U2>,
        A1: Exp + AddExp<A2>,
        L2: Exp,
        U2: Exp,
        A2: Exp,
    > AddExp<SidedDim<L2, U2, A2>> for SidedDim<L1, U1, A1>
{
    type Output = SidedDim<
        <L1 as AddExp<L2>>::Output,
        <U1 as AddExp<U2>>::Output,
        <A1 as AddExp<A2>>::Output,
    >;
}

//
// Subtraction for SidedDim
//
impl<
        L1: Exp + SubExp<L2>,
        U1: Exp + SubExp<U2>,
        A1: Exp + SubExp<A2>,
        L2: Exp,
        U2: Exp,
        A2: Exp,
    > SubExp<SidedDim<L2, U2, A2>> for SidedDim<L1, U1, A1>
{
    type Output = SidedDim<
        <L1 as SubExp<L2>>::Output,
        <U1 as SubExp<U2>>::Output,
        <A1 as SubExp<A2>>::Output,
    >;
}
