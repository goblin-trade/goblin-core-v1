use crate::quantities::{add_exp::AddExp, sub_exp::SubExp, Exp, Quantity};
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

/// Addition
impl<D: Exp> Add for Quantity<D> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner + rhs.inner)
    }
}

/// Subtraction
impl<D: Exp> Sub for Quantity<D> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner - rhs.inner)
    }
}

/// Multiplication
impl<D1: Exp, D2: Exp> Mul<Quantity<D2>> for Quantity<D1>
where
    D1: AddExp<D2>,
{
    type Output = Quantity<<D1 as AddExp<D2>>::Output>;

    fn mul(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner * rhs.inner)
    }
}

/// Division
impl<D1: Exp, D2: Exp> Div<Quantity<D2>> for Quantity<D1>
where
    D1: SubExp<D2>,
{
    type Output = Quantity<<D1 as SubExp<D2>>::Output>;

    fn div(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner / rhs.inner)
    }
}

/// Remainder or Modulo
impl<D1: Exp, D2: Exp> Rem<Quantity<D2>> for Quantity<D1> {
    type Output = Self;

    fn rem(self, rhs: Quantity<D2>) -> Self::Output {
        Quantity::new(self.inner % rhs.inner)
    }
}

/// AddAssign
impl<D: Exp> AddAssign for Quantity<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

/// SubAssign
impl<D: Exp> SubAssign for Quantity<D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner;
    }
}
