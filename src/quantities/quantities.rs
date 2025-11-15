use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};
use core::u64;

use crate::types::{Base, LegMarker, Quote};
//
// Type-level integers for exponents: -1, 0, +1
//

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

//
// Type-level addition of exponents
//
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

//
// Negation
//
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

//
// Subtraction = add negated RHS
//
pub trait SubExp<Rhs: Exp>: Exp {
    type Output: Exp;
}
impl<L: Exp + AddExp<<R as NegExp>::Output>, R: Exp + NegExp> SubExp<R> for L {
    type Output = <L as AddExp<<R as NegExp>::Output>>::Output;
}
//
// Compact sided dimension (L, U, A)
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SidedDim<L: Exp, U: Exp, A: Exp>(PhantomData<(L, U, A)>);

impl<L: Exp, U: Exp, A: Exp> Exp for SidedDim<L, U, A> {}

//
// Addition for SidedDim
//
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

//
// Full dimension = Base side, Quote side, Tick exponent
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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

//
// Subtraction for Dim
//
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

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Quantity<D: Exp> {
    pub inner: u64,
    _phantom: PhantomData<D>,
}

impl<D: Exp> Quantity<D> {
    pub const fn new(value: u64) -> Self {
        Self {
            inner: value,
            _phantom: PhantomData,
        }
    }
}

impl<D: Exp> From<u64> for Quantity<D> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// Blanket trait for all supported Quantity operations
///
pub trait QuantityOps:
    Copy + Sized + PartialEq + Default + Add<Output = Self> + Sub<Output = Self> + AddAssign + SubAssign
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

// Implementation for constants, addition and subtraction.
// These will be used in the leg namespace.
//
// Multiplication and division operations are asymmetric and happen
// in the side namespace.
impl<D> QuantityOps for Quantity<D>
where
    D: Exp + Copy + PartialEq + Default,
{
    const MIN: Self = Self::new(u64::MIN);
    const MAX: Self = Self::new(u64::MAX);
    const ZERO: Self = Self::new(0);
    const ONE: Self = Self::new(1);

    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}

//
// Multiplication
//
impl<D1: Exp, D2: Exp> Mul<Quantity<D2>> for Quantity<D1>
where
    D1: AddExp<D2>,
{
    type Output = Quantity<<D1 as AddExp<D2>>::Output>;

    fn mul(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner * rhs.inner)
    }
}

//
// Division
//
impl<D1: Exp, D2: Exp> Div<Quantity<D2>> for Quantity<D1>
where
    D1: SubExp<D2>,
{
    type Output = Quantity<<D1 as SubExp<D2>>::Output>;

    fn div(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner / rhs.inner)
    }
}

//
// Remainder or Modulo
//
impl<D1: Exp, D2: Exp> Rem<Quantity<D2>> for Quantity<D1> {
    type Output = Self;

    fn rem(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner % rhs.inner)
    }
}

//
// Addition
//
impl<D: Exp> Add for Quantity<D> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner + rhs.inner)
    }
}

//
// Subtraction
//
impl<D: Exp> Sub for Quantity<D> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner - rhs.inner)
    }
}

//
// AddAssign
//
impl<D: Exp> AddAssign for Quantity<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

//
// SubAssign
//
impl<D: Exp> SubAssign for Quantity<D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner;
    }
}

//
// Aliases
//
type BaseDim<L, U, A> = SidedDim<L, U, A>;
type QuoteDim<L, U, A> = SidedDim<L, U, A>;

pub type BaseLots = Quantity<Dim<BaseDim<P1, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseUnits = Quantity<Dim<BaseDim<Z0, P1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseAtoms = Quantity<Dim<BaseDim<Z0, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLots = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;
pub type QuoteUnits = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, P1, Z0>, Z0>>;
pub type QuoteAtoms = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, P1>, Z0>>;
pub type Ticks = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, P1>>;

// Binary ratios
pub type BaseLotsPerBaseUnit = Quantity<Dim<BaseDim<P1, N1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLotsPerQuoteUnit = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, N1, Z0>, Z0>>;
pub type QuoteLotsPerBaseUnit = Quantity<Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;

pub type BaseAtomsPerBaseUnit = Quantity<Dim<BaseDim<Z0, N1, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteUnit = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, N1, P1>, Z0>>;

pub type BaseAtomsPerBaseLot = Quantity<Dim<BaseDim<N1, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteLot = Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<N1, Z0, P1>, Z0>>;

// Tertiary
pub type QuoteLotsPerBaseUnitPerTick = Quantity<Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type QuoteLotsPerBaseLotPerTick = Quantity<Dim<BaseDim<N1, Z0, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type AdjustedQuoteLots = Quantity<Dim<BaseDim<P1, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;

// Constants
pub const BASE_ATOMS_PER_BASE_UNIT: BaseAtomsPerBaseUnit = BaseAtomsPerBaseUnit::new(1_000_000);
pub const QUOTE_ATOMS_PER_QUOTE_UNIT: QuoteAtomsPerQuoteUnit =
    QuoteAtomsPerQuoteUnit::new(1_000_000);

// Unsided units
//
// The code only uses UnsidedAtoms
//
// The definitions however allow conversions between any quantities belonging to a specific side.
// Eg. BaseLotsPerBaseUnit can be converted to sideless LotsPerUnit
pub type Unsided<L, U, A> = Quantity<SidedDim<L, U, A>>;
pub type UnsidedAtoms = Unsided<Z0, Z0, P1>;

pub trait AsUnsided<S: LegMarker, L: Exp, U: Exp, A: Exp> {
    fn unsided(self) -> Unsided<L, U, A>;
}

/// Base → Unsided
impl<L: Exp, U: Exp, A: Exp> AsUnsided<Base, L, U, A>
    for Quantity<Dim<BaseDim<L, U, A>, QuoteDim<Z0, Z0, Z0>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}

/// Quote → Unsided
impl<L: Exp, U: Exp, A: Exp> AsUnsided<Quote, L, U, A>
    for Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<L, U, A>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct DeltaAtoms {
    inner: i64,
}

impl DeltaAtoms {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let base_lots: BaseLots = Quantity::new(10);
        let base_units: BaseUnits = Quantity::new(5);
        let ticks: Ticks = Quantity::new(2);

        let more_ticks = Ticks::new(1);
        let _tick_sum = ticks + more_ticks;
        let _tick_diff = ticks - more_ticks;

        let mut mutable_ticks = ticks;
        mutable_ticks += more_ticks;
        mutable_ticks -= Ticks::new(1);

        let _lot_unit = base_lots * base_units;
        let _lot_unit_tick = _lot_unit * ticks;
        let _lot_per_unit = base_lots / base_units;

        let _checked_add = Ticks::new(1).checked_add(Ticks::new(2));
    }

    #[test]
    fn test_into_unsided() {
        let base_atoms = BaseAtoms::new(1);
        base_atoms.unsided();

        let quote_atoms = QuoteAtoms::new(1);
        quote_atoms.unsided();

        let adjusted = AdjustedQuoteLots::new(1);
    }

    #[test]
    fn test_min() {
        let a = BaseAtoms::new(1);
        let b = BaseAtoms::new(2);

        a.min(b);
    }
}
