use core::marker::PhantomData;

use crate::quantities::{Exp, add_exp::AddExp, sub_exp::SubExp};

/// Track lots, units and atoms for a base or quote limb.
///
/// Used in `Dim` to form the full dimension.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct UnsidedDim<L: Exp, U: Exp, A: Exp>(PhantomData<(L, U, A)>);

impl<L: Exp, U: Exp, A: Exp> Exp for UnsidedDim<L, U, A> {}

/// Addition for SidedDim
impl<L1: Exp + AddExp<L2>, U1: Exp + AddExp<U2>, A1: Exp + AddExp<A2>, L2: Exp, U2: Exp, A2: Exp>
    AddExp<UnsidedDim<L2, U2, A2>> for UnsidedDim<L1, U1, A1>
{
    type Output = UnsidedDim<
        <L1 as AddExp<L2>>::Output,
        <U1 as AddExp<U2>>::Output,
        <A1 as AddExp<A2>>::Output,
    >;
}

//
// Subtraction for SidedDim
//
impl<L1: Exp + SubExp<L2>, U1: Exp + SubExp<U2>, A1: Exp + SubExp<A2>, L2: Exp, U2: Exp, A2: Exp>
    SubExp<UnsidedDim<L2, U2, A2>> for UnsidedDim<L1, U1, A1>
{
    type Output = UnsidedDim<
        <L1 as SubExp<L2>>::Output,
        <U1 as SubExp<U2>>::Output,
        <A1 as SubExp<A2>>::Output,
    >;
}
