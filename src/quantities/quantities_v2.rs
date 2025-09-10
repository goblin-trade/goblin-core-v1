use core::marker::PhantomData;
use core::ops::{Div, Mul};

//
// Type-level integers for exponents: -1, 0, +1
//

#[derive(Clone, Copy, PartialEq)]
pub struct N1; // -1
#[derive(Clone, Copy, PartialEq)]
pub struct Z0; //  0
#[derive(Clone, Copy, PartialEq)]
pub struct P1; // +1

pub trait Dim {}
impl Dim for N1 {}
impl Dim for Z0 {}
impl Dim for P1 {}

//
// Type-level addition of dimensions
//
pub trait AddDim<Rhs: Dim> {
    type Output: Dim;
}

impl AddDim<Z0> for Z0 {
    type Output = Z0;
}
impl AddDim<P1> for Z0 {
    type Output = P1;
}
impl AddDim<N1> for Z0 {
    type Output = N1;
}

impl AddDim<Z0> for P1 {
    type Output = P1;
}
impl AddDim<N1> for P1 {
    type Output = Z0;
}
// P1 + P1 would be invalid → no impl

impl AddDim<Z0> for N1 {
    type Output = N1;
}
impl AddDim<P1> for N1 {
    type Output = Z0;
}
// N1 + N1 would be invalid → no impl

//
// Negation
//
pub trait NegDim {
    type Output: Dim;
}
impl NegDim for P1 {
    type Output = N1;
}
impl NegDim for N1 {
    type Output = P1;
}
impl NegDim for Z0 {
    type Output = Z0;
}

//
// Subtraction = add negated RHS
//
pub trait SubDim<Rhs: Dim>: Dim {
    type Output: Dim;
}
impl<L: Dim + AddDim<<R as NegDim>::Output>, R: Dim + NegDim> SubDim<R> for L {
    type Output = <L as AddDim<<R as NegDim>::Output>>::Output;
}

//
// Quantity type: value + 3 exponents (Lot, Unit, Tick)
//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity<V, L: Dim, U: Dim, T: Dim> {
    value: V,
    _phantom: PhantomData<(L, U, T)>,
}

impl<V, L: Dim, U: Dim, T: Dim> Quantity<V, L, U, T> {
    pub fn new(v: V) -> Self {
        Self {
            value: v,
            _phantom: PhantomData,
        }
    }
    pub fn value(&self) -> &V {
        &self.value
    }
}

//
// Multiplication
//
impl<V, L1: Dim, U1: Dim, T1: Dim, L2: Dim, U2: Dim, T2: Dim> Mul<Quantity<V, L2, U2, T2>>
    for Quantity<V, L1, U1, T1>
where
    V: Copy + Mul<Output = V>,
    L1: AddDim<L2>,
    U1: AddDim<U2>,
    T1: AddDim<T2>,
{
    type Output = Quantity<
        V,
        <L1 as AddDim<L2>>::Output,
        <U1 as AddDim<U2>>::Output,
        <T1 as AddDim<T2>>::Output,
    >;

    fn mul(self, rhs: Quantity<V, L2, U2, T2>) -> <Self as Mul<Quantity<V, L2, U2, T2>>>::Output {
        Quantity::new(self.value * rhs.value)
    }
}

//
// Division
//
impl<V, L1: Dim, U1: Dim, T1: Dim, L2: Dim, U2: Dim, T2: Dim> Div<Quantity<V, L2, U2, T2>>
    for Quantity<V, L1, U1, T1>
where
    V: Copy + Div<Output = V>,
    L1: SubDim<L2>,
    U1: SubDim<U2>,
    T1: SubDim<T2>,
{
    type Output = Quantity<
        V,
        <L1 as SubDim<L2>>::Output,
        <U1 as SubDim<U2>>::Output,
        <T1 as SubDim<T2>>::Output,
    >;

    fn div(self, rhs: Quantity<V, L2, U2, T2>) -> <Self as Div<Quantity<V, L2, U2, T2>>>::Output {
        Quantity::new(self.value / rhs.value)
    }
}

//
// Base units (you can choose numeric type)
//
type Lot<V> = Quantity<V, P1, Z0, Z0>;
type Unit<V> = Quantity<V, Z0, P1, Z0>;
type Tick<V> = Quantity<V, Z0, Z0, P1>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod() {
        let lots: Lot<u64> = Quantity::new(10);
        let units: Unit<u64> = Quantity::new(5);
        let ticks: Tick<u64> = Quantity::new(2);

        let lot_unit = lots * units; // Lot*Unit
        let lot_unit_tick = lot_unit * ticks;
        let lot_per_unit = lots / units; // Lot/Unit
    }
}
