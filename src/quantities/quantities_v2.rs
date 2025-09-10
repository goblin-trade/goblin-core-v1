use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

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
pub struct Quantity<V, D: Exp> {
    value: V,
    _phantom: PhantomData<D>,
}

impl<V, D: Exp> Quantity<V, D> {
    pub const fn new(v: V) -> Self {
        Self {
            value: v,
            _phantom: PhantomData,
        }
    }
}

//
// Multiplication
//
impl<V, D1: Exp, D2: Exp> Mul<Quantity<V, D2>> for Quantity<V, D1>
where
    V: Copy + Mul<Output = V>,
    D1: AddExp<D2>,
{
    type Output = Quantity<V, <D1 as AddExp<D2>>::Output>;

    fn mul(self, rhs: Quantity<V, D2>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

//
// Division
//
impl<V, D1: Exp, D2: Exp> Div<Quantity<V, D2>> for Quantity<V, D1>
where
    V: Copy + Div<Output = V>,
    D1: SubExp<D2>,
{
    type Output = Quantity<V, <D1 as SubExp<D2>>::Output>;

    fn div(self, rhs: Quantity<V, D2>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

//
// Addition
//
impl<V, D: Exp> Add for Quantity<V, D>
where
    V: Copy + Add<Output = V>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

//
// Subtraction
//
impl<V, D: Exp> Sub for Quantity<V, D>
where
    V: Copy + Sub<Output = V>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value - rhs.value)
    }
}

//
// AddAssign
//
impl<V, D: Exp> AddAssign for Quantity<V, D>
where
    V: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

//
// SubAssign
//
impl<V, D: Exp> SubAssign for Quantity<V, D>
where
    V: Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

//
// Checked Add/Sub
//
impl<V, D: Exp> Quantity<V, D>
where
    V: Copy + Sized,
{
    pub fn checked_add(self, rhs: Self) -> Option<Self>
    where
        V: CheckedAdd,
    {
        self.value.checked_add(rhs.value).map(Quantity::new)
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self>
    where
        V: CheckedSub,
    {
        self.value.checked_sub(rhs.value).map(Quantity::new)
    }
}

/// Trait for types that support checked addition
pub trait CheckedAdd: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
}

/// Trait for types that support checked subtraction
pub trait CheckedSub: Sized {
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

impl CheckedAdd for i64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        i64::checked_add(self, rhs)
    }
}

impl CheckedSub for i64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        i64::checked_sub(self, rhs)
    }
}

impl CheckedAdd for u64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        u64::checked_add(self, rhs)
    }
}

impl CheckedSub for u64 {
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

        let checked_add = Tick::new(1).checked_add(Tick::new(2));
    }
}
