use crate::quantities::{add_exp::AddExp, sub_exp::SubExp, Exp, Quantity, QuantityOps};
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

/// Addition
impl<E, I> Add for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner + rhs.inner)
    }
}

/// Subtraction
impl<E, I> Sub for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner - rhs.inner)
    }
}

/// Multiplication
impl<E0, E1, I> Mul<Quantity<E1, I>> for Quantity<E0, I>
where
    E0: Exp + AddExp<E1>,
    E1: Exp,
    I: QuantityOps + Mul<Output = I>,
{
    type Output = Quantity<<E0 as AddExp<E1>>::Output, I>;

    fn mul(self, rhs: Quantity<E1, I>) -> Self::Output {
        Quantity::new(self.inner * rhs.inner)
    }
}

impl<E0, I> Quantity<E0, I>
where
    E0: Exp,
    I: QuantityOps,
{
    pub fn checked_mul<E1>(
        self,
        rhs: Quantity<E1, I>,
    ) -> Option<Quantity<<E0 as AddExp<E1>>::Output, I>>
    where
        E0: AddExp<E1>,
        E1: Exp,
    {
        self.inner.checked_mul(rhs.inner).map(Quantity::new)
    }
}

/// Division
impl<E0, E1, I> Div<Quantity<E1, I>> for Quantity<E0, I>
where
    E0: Exp + SubExp<E1>,
    E1: Exp,
    I: QuantityOps + Div<Output = I>,
{
    type Output = Quantity<<E0 as SubExp<E1>>::Output, I>;

    fn div(self, rhs: Quantity<E1, I>) -> Self::Output {
        Quantity::new(self.inner / rhs.inner)
    }
}

/// Remainder or Modulo
impl<E0, E1, I> Rem<Quantity<E1, I>> for Quantity<E0, I>
where
    E0: Exp,
    E1: Exp,
    I: QuantityOps + Rem<Output = I>,
{
    type Output = Self;

    fn rem(self, rhs: Quantity<E1, I>) -> Self::Output {
        Quantity::new(self.inner % rhs.inner)
    }
}

/// AddAssign
impl<E, I> AddAssign for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

/// SubAssign
impl<E, I> SubAssign for Quantity<E, I>
where
    E: Exp,
    I: QuantityOps,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner;
    }
}
