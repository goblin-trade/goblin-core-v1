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
// Quantity type: value + 7 exponents (BaseLots, BaseUnits, BaseAtoms, QuoteLots, QuoteUnits, QuoteAtoms, Tick)
//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity<
    V,
    BaseLotsExp: Exp,
    BaseUnitsExp: Exp,
    BaseAtomsExp: Exp,
    QuoteLotsExp: Exp,
    QuoteUnitsExp: Exp,
    QuoteAtomsExp: Exp,
    TickExp: Exp,
> {
    value: V,
    _phantom: PhantomData<(
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    )>,
}

impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
{
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
impl<
        V,
        BaseLotsExp1: Exp,
        BaseUnitsExp1: Exp,
        BaseAtomsExp1: Exp,
        QuoteLotsExp1: Exp,
        QuoteUnitsExp1: Exp,
        QuoteAtomsExp1: Exp,
        TickExp1: Exp,
        BaseLotsExp2: Exp,
        BaseUnitsExp2: Exp,
        BaseAtomsExp2: Exp,
        QuoteLotsExp2: Exp,
        QuoteUnitsExp2: Exp,
        QuoteAtomsExp2: Exp,
        TickExp2: Exp,
    >
    Mul<
        Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp1,
        BaseUnitsExp1,
        BaseAtomsExp1,
        QuoteLotsExp1,
        QuoteUnitsExp1,
        QuoteAtomsExp1,
        TickExp1,
    >
where
    V: Copy + Mul<Output = V>,
    BaseLotsExp1: AddExp<BaseLotsExp2>,
    BaseUnitsExp1: AddExp<BaseUnitsExp2>,
    BaseAtomsExp1: AddExp<BaseAtomsExp2>,
    QuoteLotsExp1: AddExp<QuoteLotsExp2>,
    QuoteUnitsExp1: AddExp<QuoteUnitsExp2>,
    QuoteAtomsExp1: AddExp<QuoteAtomsExp2>,
    TickExp1: AddExp<TickExp2>,
{
    type Output = Quantity<
        V,
        <BaseLotsExp1 as AddExp<BaseLotsExp2>>::Output,
        <BaseUnitsExp1 as AddExp<BaseUnitsExp2>>::Output,
        <BaseAtomsExp1 as AddExp<BaseAtomsExp2>>::Output,
        <QuoteLotsExp1 as AddExp<QuoteLotsExp2>>::Output,
        <QuoteUnitsExp1 as AddExp<QuoteUnitsExp2>>::Output,
        <QuoteAtomsExp1 as AddExp<QuoteAtomsExp2>>::Output,
        <TickExp1 as AddExp<TickExp2>>::Output,
    >;

    fn mul(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    ) -> <Self as Mul<
        Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    >>::Output {
        Quantity::new(self.value * rhs.value)
    }
}

//
// Division
//
impl<
        V,
        BaseLotsExp1: Exp,
        BaseUnitsExp1: Exp,
        BaseAtomsExp1: Exp,
        QuoteLotsExp1: Exp,
        QuoteUnitsExp1: Exp,
        QuoteAtomsExp1: Exp,
        TickExp1: Exp,
        BaseLotsExp2: Exp,
        BaseUnitsExp2: Exp,
        BaseAtomsExp2: Exp,
        QuoteLotsExp2: Exp,
        QuoteUnitsExp2: Exp,
        QuoteAtomsExp2: Exp,
        TickExp2: Exp,
    >
    Div<
        Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp1,
        BaseUnitsExp1,
        BaseAtomsExp1,
        QuoteLotsExp1,
        QuoteUnitsExp1,
        QuoteAtomsExp1,
        TickExp1,
    >
where
    V: Copy + Div<Output = V>,
    BaseLotsExp1: SubExp<BaseLotsExp2>,
    BaseUnitsExp1: SubExp<BaseUnitsExp2>,
    BaseAtomsExp1: SubExp<BaseAtomsExp2>,
    QuoteLotsExp1: SubExp<QuoteLotsExp2>,
    QuoteUnitsExp1: SubExp<QuoteUnitsExp2>,
    QuoteAtomsExp1: SubExp<QuoteAtomsExp2>,
    TickExp1: SubExp<TickExp2>,
{
    type Output = Quantity<
        V,
        <BaseLotsExp1 as SubExp<BaseLotsExp2>>::Output,
        <BaseUnitsExp1 as SubExp<BaseUnitsExp2>>::Output,
        <BaseAtomsExp1 as SubExp<BaseAtomsExp2>>::Output,
        <QuoteLotsExp1 as SubExp<QuoteLotsExp2>>::Output,
        <QuoteUnitsExp1 as SubExp<QuoteUnitsExp2>>::Output,
        <QuoteAtomsExp1 as SubExp<QuoteAtomsExp2>>::Output,
        <TickExp1 as SubExp<TickExp2>>::Output,
    >;

    fn div(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    ) -> <Self as Div<
        Quantity<
            V,
            BaseLotsExp2,
            BaseUnitsExp2,
            BaseAtomsExp2,
            QuoteLotsExp2,
            QuoteUnitsExp2,
            QuoteAtomsExp2,
            TickExp2,
        >,
    >>::Output {
        Quantity::new(self.value / rhs.value)
    }
}

//
// Addition
//
impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    Add<
        Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
where
    V: Copy + Add<Output = V>,
{
    type Output = Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >;

    fn add(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

//
// Subtraction
//
impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    Sub<
        Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
where
    V: Copy + Sub<Output = V>,
{
    type Output = Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >;

    fn sub(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) -> Self::Output {
        Quantity::new(self.value - rhs.value)
    }
}

//
// AddAssign
//
impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    AddAssign<
        Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
where
    V: Copy + AddAssign,
{
    fn add_assign(
        &mut self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) {
        self.value += rhs.value;
    }
}

//
// SubAssign
//
impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    SubAssign<
        Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    >
    for Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
where
    V: Copy + SubAssign,
{
    fn sub_assign(
        &mut self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) {
        self.value -= rhs.value;
    }
}

impl<
        V,
        BaseLotsExp: Exp,
        BaseUnitsExp: Exp,
        BaseAtomsExp: Exp,
        QuoteLotsExp: Exp,
        QuoteUnitsExp: Exp,
        QuoteAtomsExp: Exp,
        TickExp: Exp,
    >
    Quantity<
        V,
        BaseLotsExp,
        BaseUnitsExp,
        BaseAtomsExp,
        QuoteLotsExp,
        QuoteUnitsExp,
        QuoteAtomsExp,
        TickExp,
    >
where
    V: Copy,
{
    /// Checked addition that returns `None` on overflow
    pub fn checked_add(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) -> Option<Self>
    where
        V: CheckedAdd<Output = V>,
    {
        self.value.checked_add(rhs.value).map(|v| Quantity::new(v))
    }

    /// Checked subtraction that returns `None` on underflow
    pub fn checked_sub(
        self,
        rhs: Quantity<
            V,
            BaseLotsExp,
            BaseUnitsExp,
            BaseAtomsExp,
            QuoteLotsExp,
            QuoteUnitsExp,
            QuoteAtomsExp,
            TickExp,
        >,
    ) -> Option<Self>
    where
        V: CheckedSub<Output = V>,
    {
        self.value.checked_sub(rhs.value).map(|v| Quantity::new(v))
    }
}

/// Trait for types that support checked addition
pub trait CheckedAdd {
    type Output;
    fn checked_add(self, rhs: Self) -> Option<Self::Output>;
}

/// Trait for types that support checked subtraction
pub trait CheckedSub {
    type Output;
    fn checked_sub(self, rhs: Self) -> Option<Self::Output>;
}

impl CheckedAdd for i64 {
    type Output = i64;
    fn checked_add(self, rhs: Self) -> Option<Self::Output> {
        i64::checked_add(self, rhs)
    }
}
impl CheckedSub for i64 {
    type Output = i64;
    fn checked_sub(self, rhs: Self) -> Option<Self::Output> {
        i64::checked_sub(self, rhs)
    }
}

impl CheckedAdd for u64 {
    type Output = u64;
    fn checked_add(self, rhs: Self) -> Option<Self::Output> {
        u64::checked_add(self, rhs)
    }
}
impl CheckedSub for u64 {
    type Output = u64;
    fn checked_sub(self, rhs: Self) -> Option<Self::Output> {
        u64::checked_sub(self, rhs)
    }
}

//
// Base units (you can choose numeric type)
//
type BaseLots = Quantity<u64, P1, Z0, Z0, Z0, Z0, Z0, Z0>;
type BaseUnits = Quantity<u64, Z0, P1, Z0, Z0, Z0, Z0, Z0>;
type BaseAtoms = Quantity<u64, Z0, Z0, P1, Z0, Z0, Z0, Z0>;
type QuoteLots = Quantity<u64, Z0, Z0, Z0, P1, Z0, Z0, Z0>;
type QuoteUnits = Quantity<u64, Z0, Z0, Z0, Z0, P1, Z0, Z0>;
type QuoteAtoms = Quantity<u64, Z0, Z0, Z0, Z0, Z0, P1, Z0>;
type Tick = Quantity<u64, Z0, Z0, Z0, Z0, Z0, Z0, P1>;

// Binary
type BaseLotsPerBaseUnit = Quantity<u64, P1, N1, Z0, Z0, Z0, Z0, Z0>;
type QuoteLotsPerQuoteUnit = Quantity<u64, Z0, Z0, Z0, P1, N1, Z0, Z0>;
type QuoteLotsPerBaseUnit = Quantity<u64, Z0, N1, Z0, P1, Z0, Z0, Z0>;

type BaseAtomsPerBaseUnit = Quantity<u64, Z0, N1, P1, Z0, Z0, Z0, Z0>;
type QuoteAtomsPerQuoteUnit = Quantity<u64, Z0, Z0, Z0, Z0, N1, P1, Z0>;

type BaseAtomsPerBaseLot = Quantity<u64, N1, Z0, P1, Z0, Z0, Z0, Z0>;
type QuoteAtomsPerQuoteLot = Quantity<u64, Z0, Z0, Z0, N1, Z0, P1, Z0>;

// Tertiary
type QuoteLotsPerBaseUnitPerTick = Quantity<u64, Z0, N1, Z0, P1, Z0, Z0, N1>;
type QuoteLotsPerBaseLotPerTick = Quantity<u64, N1, Z0, Z0, P1, Z0, Z0, N1>;
type AdjustedQuoteLots = Quantity<u64, P1, N1, Z0, P1, Z0, Z0, Z0>;

const BASE_ATOMS_PER_BASE_UNIT: BaseAtomsPerBaseUnit = BaseAtomsPerBaseUnit::new(1_000_000);
const QUOTE_ATOMS_PER_QUOTE_UNIT: QuoteAtomsPerQuoteUnit = QuoteAtomsPerQuoteUnit::new(1_000_000);

// Delta types
type BaseLotsDelta = Quantity<i64, P1, Z0, Z0, Z0, Z0, Z0, Z0>;
type QuoteLotsDelta = Quantity<i64, Z0, Z0, Z0, P1, Z0, Z0, Z0>;
type BaseAtomsDelta = Quantity<i64, Z0, Z0, P1, Z0, Z0, Z0, Z0>;
type QuoteAtomsDelta = Quantity<i64, Z0, Z0, Z0, Z0, Z0, P1, Z0>;

// TODO implement conversion between u64 and i64 variants
// - u64 to i64: two functions for positive and negative value. Return type Result<X, GoblinError>
// Return GoblinError::DeltaOverflow if value doesn't fit in i64.
//
// - i64 to u64: abs() function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prod() {
        let base_lots: BaseLots = Quantity::new(10);
        let base_units: BaseUnits = Quantity::new(5);
        let ticks: Tick = Quantity::new(2);

        // Addition and subtraction work for quantities with same dimensions
        let more_ticks = Tick::new(1);
        let _tick_sum = ticks + more_ticks; // Tick + Tick = Tick
        let _tick_diff = ticks - more_ticks; // Tick - Tick = Tick

        // AddAssign and SubAssign
        let mut mutable_ticks = ticks;
        mutable_ticks += more_ticks;
        mutable_ticks -= Tick::new(1);

        let _lot_unit = base_lots * base_units; // BaseLots*BaseUnits
        let _lot_unit_tick = _lot_unit * ticks;
        let _lot_per_unit = base_lots / base_units; // BaseLots/BaseUnits
    }
}
