use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

use crate::goblin_error::GoblinError;
use crate::require;

//
// Type-level integers for exponents: -1, 0, +1
//

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct N1; // -1
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Z0; //  0
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
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
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
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
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
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
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<V: Copy + Sized + Numeric, D: Exp> {
    pub inner: V,
    _phantom: PhantomData<D>,
}

impl<V: Copy + Sized + Numeric, D: Exp> Quantity<V, D> {
    pub const fn new(v: V) -> Self {
        Self {
            inner: v,
            _phantom: PhantomData,
        }
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
impl<V, D> QuantityOps for Quantity<V, D>
where
    V: Copy
        + Numeric
        + Default
        + Add<Output = V>
        + Sub<Output = V>
        + AddAssign
        + SubAssign
        + PartialEq,
    D: Exp + Copy + PartialEq + Default,
{
    const MIN: Self = Self::new(V::MIN);
    const MAX: Self = Self::new(V::MAX);
    const ZERO: Self = Self::new(V::ZERO);
    const ONE: Self = Self::new(V::ONE);

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
// Remainder or Modulo
//
impl<V, D1: Exp, D2: Exp> Rem<Quantity<V, D2>> for Quantity<V, D1>
where
    V: Copy + Rem<Output = V> + Numeric,
{
    type Output = Self;

    fn rem(self, rhs: Quantity<V, D2>) -> Self::Output {
        Quantity::new(self.inner % rhs.inner)
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
// Aliases
//
type BaseDim<L, U, A> = SidedDim<L, U, A>;
type QuoteDim<L, U, A> = SidedDim<L, U, A>;

pub type BaseLots = Quantity<u64, Dim<BaseDim<P1, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseUnits = Quantity<u64, Dim<BaseDim<Z0, P1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type BaseAtoms = Quantity<u64, Dim<BaseDim<Z0, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLots = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;
pub type QuoteUnits = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, P1, Z0>, Z0>>;
pub type QuoteAtoms = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, P1>, Z0>>;
pub type Ticks = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, P1>>;

// Binary ratios
pub type BaseLotsPerBaseUnit = Quantity<u64, Dim<BaseDim<P1, N1, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLotsPerQuoteUnit = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, N1, Z0>, Z0>>;
pub type QuoteLotsPerBaseUnit = Quantity<u64, Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;

pub type BaseAtomsPerBaseUnit = Quantity<u64, Dim<BaseDim<Z0, N1, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteUnit = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, N1, P1>, Z0>>;

pub type BaseAtomsPerBaseLot = Quantity<u64, Dim<BaseDim<N1, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsPerQuoteLot = Quantity<u64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<N1, Z0, P1>, Z0>>;

// Tertiary
pub type QuoteLotsPerBaseUnitPerTick =
    Quantity<u64, Dim<BaseDim<Z0, N1, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type QuoteLotsPerBaseLotPerTick =
    Quantity<u64, Dim<BaseDim<N1, Z0, Z0>, QuoteDim<P1, Z0, Z0>, N1>>;
pub type AdjustedQuoteLots = Quantity<u64, Dim<BaseDim<P1, N1, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;

pub const BASE_ATOMS_PER_BASE_UNIT: BaseAtomsPerBaseUnit = BaseAtomsPerBaseUnit::new(1_000_000);
pub const QUOTE_ATOMS_PER_QUOTE_UNIT: QuoteAtomsPerQuoteUnit =
    QuoteAtomsPerQuoteUnit::new(1_000_000);

// Delta types
pub type BaseLotsDelta = Quantity<i64, Dim<BaseDim<P1, Z0, Z0>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteLotsDelta = Quantity<i64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<P1, Z0, Z0>, Z0>>;
pub type BaseAtomsDelta = Quantity<i64, Dim<BaseDim<Z0, Z0, P1>, QuoteDim<Z0, Z0, Z0>, Z0>>;
pub type QuoteAtomsDelta = Quantity<i64, Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<Z0, Z0, P1>, Z0>>;

// Try to convert unsigned to delta. Return error if overflow
impl<D: Exp> TryFrom<Quantity<u64, D>> for Quantity<i64, D> {
    type Error = GoblinError;

    fn try_from(value: Quantity<u64, D>) -> Result<Self, Self::Error> {
        require!(value.inner <= i64::MAX as u64, GoblinError::DeltaOverflow);
        Ok(Quantity::new(value.inner as i64))
    }
}

// Convert delta to unsigned. Use absolute value
impl<D: Exp> From<Quantity<i64, D>> for Quantity<u64, D> {
    fn from(value: Quantity<i64, D>) -> Self {
        Quantity::new(value.inner.unsigned_abs())
    }
}

// Multiplying unsigned with signed delta
// Exponent rules remain the same
// We just cast the unsigned to signed and then do the operation
// i64::try_from(u64)
// Instead of returning Quantity, return Result or Option type

//
// Multiply Delta with an unsigned quantity
// The result is also a delta
impl<D1: Exp, D2: Exp> Mul<Quantity<u64, D2>> for Quantity<i64, D1>
where
    D1: AddExp<D2>,
{
    type Output = Quantity<i64, <D1 as AddExp<D2>>::Output>;

    fn mul(self, rhs: Quantity<u64, D2>) -> Self::Output {
        let right_value: i64 = rhs.inner as i64;
        Quantity::new(self.inner * right_value)
    }
}

// impl<D1: Exp, D2: Exp> Mul<Quantity<i64, D2>> for Quantity<u64, D1>
// where
//     D1: AddExp<D2>,
// {
//     type Output = Quantity<i64, <D1 as AddExp<D2>>::Output>;

//     fn mul(self, rhs: Quantity<i64, D2>) -> Self::Output {
//         let left_value: i64 = self.inner as i64;
//         Quantity::new(left_value * rhs.inner)
//     }
// }

// impl<V1, V2, D1: Exp, D2: Exp> Mul<Quantity<V2, D2>> for Quantity<V1, D1>
// where
//     V1: Copy + Mul<V2, Output = V1> + Numeric,
//     // V2: Copy + Mul<Output = V> + Numeric,
//     D1: AddExp<D2>,
// {
//     type Output = Quantity<V1, <D1 as AddExp<D2>>::Output>;

//     fn mul(self, rhs: Quantity<V2, D2>) -> Self::Output {
//         // Inner type of RHS (V2) must be cast to inner type of self V1
//         Quantity::new(self.inner)
//         // Quantity::new(self.inner * rhs.inner)
//     }
// }

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
    fn test_mod() {
        let lot_size = BaseLotsPerBaseUnit::new(1);

        // This works here. I need mod to work via LegMarker trait
        let zz = BASE_ATOMS_PER_BASE_UNIT % lot_size;
    }

    #[test]
    fn test_cast() {
        let a = 10u64;
        let b = a as i64;
        let c = i64::try_from(a);
    }

    #[test]
    fn test_delta_mul() {
        let lots = BaseLots::new(1);
        let lots_delta = BaseLotsDelta::new(1);
        let atoms_per_lot = BaseAtomsPerBaseLot::new(2);

        let atoms = lots * atoms_per_lot;
        let atoms_delta = lots_delta * atoms_per_lot;
    }
}
