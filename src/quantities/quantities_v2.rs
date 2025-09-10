use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::goblin_error::GoblinError;
use crate::require;

//
// Type-level integers for exponents: -1, 0, +1
//

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct N1; // -1
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Z0; //  0
#[derive(Clone, Copy, PartialEq, Debug)]
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
// Dimension wrapper: groups the 7 exponents
//
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Dim<BL: Exp, BU: Exp, BA: Exp, QL: Exp, QU: Exp, QA: Exp, T: Exp>(
    PhantomData<(BL, BU, BA, QL, QU, QA, T)>,
);

impl<BL: Exp, BU: Exp, BA: Exp, QL: Exp, QU: Exp, QA: Exp, T: Exp> Exp
    for Dim<BL, BU, BA, QL, QU, QA, T>
{
}

//
// Addition of Dims
//
impl<
        BL1: Exp + AddExp<BL2>,
        BU1: Exp + AddExp<BU2>,
        BA1: Exp + AddExp<BA2>,
        QL1: Exp + AddExp<QL2>,
        QU1: Exp + AddExp<QU2>,
        QA1: Exp + AddExp<QA2>,
        T1: Exp + AddExp<T2>,
        BL2: Exp,
        BU2: Exp,
        BA2: Exp,
        QL2: Exp,
        QU2: Exp,
        QA2: Exp,
        T2: Exp,
    > AddExp<Dim<BL2, BU2, BA2, QL2, QU2, QA2, T2>> for Dim<BL1, BU1, BA1, QL1, QU1, QA1, T1>
{
    type Output = Dim<
        <BL1 as AddExp<BL2>>::Output,
        <BU1 as AddExp<BU2>>::Output,
        <BA1 as AddExp<BA2>>::Output,
        <QL1 as AddExp<QL2>>::Output,
        <QU1 as AddExp<QU2>>::Output,
        <QA1 as AddExp<QA2>>::Output,
        <T1 as AddExp<T2>>::Output,
    >;
}

//
// Subtraction of Dims
//
impl<
        BL1: Exp + SubExp<BL2>,
        BU1: Exp + SubExp<BU2>,
        BA1: Exp + SubExp<BA2>,
        QL1: Exp + SubExp<QL2>,
        QU1: Exp + SubExp<QU2>,
        QA1: Exp + SubExp<QA2>,
        T1: Exp + SubExp<T2>,
        BL2: Exp,
        BU2: Exp,
        BA2: Exp,
        QL2: Exp,
        QU2: Exp,
        QA2: Exp,
        T2: Exp,
    > SubExp<Dim<BL2, BU2, BA2, QL2, QU2, QA2, T2>> for Dim<BL1, BU1, BA1, QL1, QU1, QA1, T1>
{
    type Output = Dim<
        <BL1 as SubExp<BL2>>::Output,
        <BU1 as SubExp<BU2>>::Output,
        <BA1 as SubExp<BA2>>::Output,
        <QL1 as SubExp<QL2>>::Output,
        <QU1 as SubExp<QU2>>::Output,
        <QA1 as SubExp<QA2>>::Output,
        <T1 as SubExp<T2>>::Output,
    >;
}

//
// Quantity type: value + Dim
//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity<V: Copy + Sized + Numeric, D: Exp> {
    pub inner: V,
    _phantom: PhantomData<D>,
}

impl<V: Copy + Sized + Numeric, D: Exp> Quantity<V, D> {
    pub const MIN: Self = Self::new(V::MIN);
    pub const MAX: Self = Self::new(V::MAX);
    pub const ZERO: Self = Self::new(V::ZERO);
    pub const ONE: Self = Self::new(V::ONE);

    pub const fn new(v: V) -> Self {
        Self {
            inner: v,
            _phantom: PhantomData,
        }
    }

    // Checked Add/Sub

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(Quantity::new)
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.inner.checked_sub(rhs.inner).map(Quantity::new)
    }
}

//
// Multiplication
//
impl<V, D1: Exp, D2: Exp> Mul<Quantity<V, D2>> for Quantity<V, D1>
where
    V: Copy + Mul<Output = V> + Numeric,
    D1: AddExp<D2>,
{
    type Output = Quantity<V, <D1 as AddExp<D2>>::Output>;

    fn mul(self, rhs: Quantity<V, D2>) -> Self::Output {
        Quantity::new(self.inner * rhs.inner)
    }
}

//
// Division
//
impl<V, D1: Exp, D2: Exp> Div<Quantity<V, D2>> for Quantity<V, D1>
where
    V: Copy + Div<Output = V> + Numeric,
    D1: SubExp<D2>,
{
    type Output = Quantity<V, <D1 as SubExp<D2>>::Output>;

    fn div(self, rhs: Quantity<V, D2>) -> Self::Output {
        Quantity::new(self.inner / rhs.inner)
    }
}

//
// Addition
//
impl<V, D: Exp> Add for Quantity<V, D>
where
    V: Copy + Add<Output = V> + Numeric,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner + rhs.inner)
    }
}

//
// Subtraction
//
impl<V, D: Exp> Sub for Quantity<V, D>
where
    V: Copy + Sub<Output = V> + Numeric,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner - rhs.inner)
    }
}

//
// AddAssign
//
impl<V, D: Exp> AddAssign for Quantity<V, D>
where
    V: Copy + AddAssign + Numeric,
{
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

//
// SubAssign
//
impl<V, D: Exp> SubAssign for Quantity<V, D>
where
    V: Copy + SubAssign + Numeric,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner;
    }
}

/// Trait for numeric types that support checked arithmetic operations
pub trait Numeric: Sized {
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

impl Numeric for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;
    const ZERO: Self = 0i64;
    const ONE: Self = 1i64;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        i64::checked_add(self, rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        i64::checked_sub(self, rhs)
    }
}

impl Numeric for u64 {
    const MIN: Self = u64::MIN;
    const MAX: Self = u64::MAX;
    const ZERO: Self = 0u64;
    const ONE: Self = 1u64;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        u64::checked_add(self, rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        u64::checked_sub(self, rhs)
    }
}

//
// Base units (you can choose numeric type)
//
type BaseLots = Quantity<u64, Dim<P1, Z0, Z0, Z0, Z0, Z0, Z0>>;
type BaseUnits = Quantity<u64, Dim<Z0, P1, Z0, Z0, Z0, Z0, Z0>>;
type BaseAtoms = Quantity<u64, Dim<Z0, Z0, P1, Z0, Z0, Z0, Z0>>;
type QuoteLots = Quantity<u64, Dim<Z0, Z0, Z0, P1, Z0, Z0, Z0>>;
type QuoteUnits = Quantity<u64, Dim<Z0, Z0, Z0, Z0, P1, Z0, Z0>>;
type QuoteAtoms = Quantity<u64, Dim<Z0, Z0, Z0, Z0, Z0, P1, Z0>>;
type Tick = Quantity<u64, Dim<Z0, Z0, Z0, Z0, Z0, Z0, P1>>;

// Binary ratios
type BaseLotsPerBaseUnit = Quantity<u64, Dim<P1, N1, Z0, Z0, Z0, Z0, Z0>>;
type QuoteLotsPerQuoteUnit = Quantity<u64, Dim<Z0, Z0, Z0, P1, N1, Z0, Z0>>;
type QuoteLotsPerBaseUnit = Quantity<u64, Dim<Z0, N1, Z0, P1, Z0, Z0, Z0>>;

type BaseAtomsPerBaseUnit = Quantity<u64, Dim<Z0, N1, P1, Z0, Z0, Z0, Z0>>;
type QuoteAtomsPerQuoteUnit = Quantity<u64, Dim<Z0, Z0, Z0, Z0, N1, P1, Z0>>;

type BaseAtomsPerBaseLot = Quantity<u64, Dim<N1, Z0, P1, Z0, Z0, Z0, Z0>>;
type QuoteAtomsPerQuoteLot = Quantity<u64, Dim<Z0, Z0, Z0, N1, Z0, P1, Z0>>;

// Tertiary
type QuoteLotsPerBaseUnitPerTick = Quantity<u64, Dim<Z0, N1, Z0, P1, Z0, Z0, N1>>;
type QuoteLotsPerBaseLotPerTick = Quantity<u64, Dim<N1, Z0, Z0, P1, Z0, Z0, N1>>;
type AdjustedQuoteLots = Quantity<u64, Dim<P1, N1, Z0, P1, Z0, Z0, Z0>>;

const BASE_ATOMS_PER_BASE_UNIT: BaseAtomsPerBaseUnit = BaseAtomsPerBaseUnit::new(1_000_000);
const QUOTE_ATOMS_PER_QUOTE_UNIT: QuoteAtomsPerQuoteUnit = QuoteAtomsPerQuoteUnit::new(1_000_000);

// Delta types
type BaseLotsDelta = Quantity<i64, Dim<P1, Z0, Z0, Z0, Z0, Z0, Z0>>;
type QuoteLotsDelta = Quantity<i64, Dim<Z0, Z0, Z0, P1, Z0, Z0, Z0>>;
type BaseAtomsDelta = Quantity<i64, Dim<Z0, Z0, P1, Z0, Z0, Z0, Z0>>;
type QuoteAtomsDelta = Quantity<i64, Dim<Z0, Z0, Z0, Z0, Z0, P1, Z0>>;

// Try to convert unsigned to delta. Return error if we overflow the bounds of i64 delta
impl<D: Exp> TryFrom<Quantity<u64, D>> for Quantity<i64, D> {
    type Error = GoblinError;

    fn try_from(value: Quantity<u64, D>) -> Result<Self, Self::Error> {
        require!(value.inner <= i64::MAX as u64, GoblinError::DeltaOverflow);
        Ok(Quantity::new(value.inner as i64))
    }
}

// Convert delta to unsigned. Use the absolute unsigned value.
impl<D: Exp> From<Quantity<i64, D>> for Quantity<u64, D> {
    fn from(value: Quantity<i64, D>) -> Self {
        Quantity::new(value.inner.unsigned_abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let base_lots: BaseLots = Quantity::new(10);
        let base_units: BaseUnits = Quantity::new(5);
        let ticks: Tick = Quantity::new(2);

        // Addition and subtraction work for quantities with same dimensions
        let more_ticks = Tick::new(1);
        let _tick_sum = ticks + more_ticks;
        let _tick_diff = ticks - more_ticks;

        // AddAssign and SubAssign
        let mut mutable_ticks = ticks;
        mutable_ticks += more_ticks;
        mutable_ticks -= Tick::new(1);

        let _lot_unit = base_lots * base_units;
        let _lot_unit_tick = _lot_unit * ticks;
        let _lot_per_unit = base_lots / base_units;

        let _checked_add = Tick::new(1).checked_add(Tick::new(2));
    }

    #[test]
    fn test_numeric_trait() {
        let tick1 = Tick::new(5);
        let tick2 = Tick::new(3);

        // Test checked_add
        let add_result = tick1.checked_add(tick2);
        assert_eq!(add_result, Some(Tick::new(8)));

        // Test checked_sub
        let sub_result = tick1.checked_sub(tick2);
        assert_eq!(sub_result, Some(Tick::new(2)));

        // Test overflow behavior
        let max_tick = Tick::new(u64::MAX);
        let overflow_result = max_tick.checked_add(Tick::new(1));
        assert_eq!(overflow_result, None);

        // Test underflow behavior
        let min_tick = Tick::new(0);
        let underflow_result = min_tick.checked_sub(Tick::new(1));
        assert_eq!(underflow_result, None);

        let max_tick = Tick::MAX;
        assert_eq!(max_tick, Tick::new(u64::MAX));
    }
}
