///! Subtraction = add negated RHS
use crate::quantities::{add_exp::AddExp, neg_exp::NegExp, Exp};

pub trait SubExp<Rhs: Exp>: Exp {
    type Output: Exp;
}
impl<L: Exp + AddExp<<R as NegExp>::Output>, R: Exp + NegExp> SubExp<R> for L {
    type Output = <L as AddExp<<R as NegExp>::Output>>::Output;
}
