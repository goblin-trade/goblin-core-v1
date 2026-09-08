//! Type-level addition of exponents
use crate::quantities::{Exp, N1, P1, Z0};

pub trait AddExp<Rhs: Exp> {
    type Output: Exp;
}

impl AddExp<Z0> for Z0 {
    type Output = Z0;
}
impl AddExp<P1> for Z0 {
    type Output = P1;
}
impl AddExp<N1> for Z0 {
    type Output = N1;
}

impl AddExp<Z0> for P1 {
    type Output = P1;
}
impl AddExp<N1> for P1 {
    type Output = Z0;
}
// P1 + P1 would be invalid → no impl

impl AddExp<Z0> for N1 {
    type Output = N1;
}
impl AddExp<P1> for N1 {
    type Output = Z0;
}
// N1 + N1 would be invalid → no impl
