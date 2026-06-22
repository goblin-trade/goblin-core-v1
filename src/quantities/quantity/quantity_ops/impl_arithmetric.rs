use crate::quantities::{add_exp::AddExp, sub_exp::SubExp, Exp, Quantity};
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

/// Addition
impl<E: Exp> Add for Quantity<E> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner + rhs.inner)
    }
}

/// Subtraction
impl<E: Exp> Sub for Quantity<E> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.inner - rhs.inner)
    }
}

/// Multiplication
impl<E0: Exp, E1: Exp> Mul<Quantity<E1>> for Quantity<E0>
where
    E0: AddExp<E1>,
{
    type Output = Quantity<<E0 as AddExp<E1>>::Output>;

    fn mul(self, rhs: Quantity<E1>) -> Self::Output {
        Quantity::new(self.inner * rhs.inner)
    }
}

/// Division
impl<E0: Exp, E1: Exp> Div<Quantity<E1>> for Quantity<E0>
where
    E0: SubExp<E1>,
{
    type Output = Quantity<<E0 as SubExp<E1>>::Output>;

    fn div(self, rhs: Quantity<E1>) -> Self::Output {
        Quantity::new(self.inner / rhs.inner)
    }
}

/// Remainder or Modulo
impl<E0: Exp, E1: Exp> Rem<Quantity<E1>> for Quantity<E0> {
    type Output = Self;

    fn rem(self, rhs: Quantity<E1>) -> Self::Output {
        Quantity::new(self.inner % rhs.inner)
    }
}

/// AddAssign
impl<E: Exp> AddAssign for Quantity<E> {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

/// SubAssign
impl<E: Exp> SubAssign for Quantity<E> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner;
    }
}
