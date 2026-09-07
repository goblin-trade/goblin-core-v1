///! Negation
use crate::quantities::{Exp, N1, P1, Z0};

pub trait NegExp {
    type Output: Exp;
}
impl NegExp for P1 {
    type Output = N1;
}
impl NegExp for N1 {
    type Output = P1;
}
impl NegExp for Z0 {
    type Output = Z0;
}
