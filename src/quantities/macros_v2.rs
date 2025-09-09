#[macro_export]
macro_rules! define_legged_type {
    ($type:ident<$t:ty>) => {
        #[repr(C)]
        #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $type<L: LegMarker>(pub $t, pub core::marker::PhantomData<L>);

        impl<L: LegMarker> From<$t> for $type<L> {
            fn from(value: $t) -> Self {
                $type(value, core::marker::PhantomData)
            }
        }

        impl<L: LegMarker> core::ops::Add for $type<L> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $type(self.0 + rhs.0, core::marker::PhantomData)
            }
        }

        impl<L: LegMarker> core::ops::AddAssign for $type<L> {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl<L: LegMarker> core::ops::Sub for $type<L> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $type(self.0 - rhs.0, core::marker::PhantomData)
            }
        }

        impl<L: LegMarker> core::ops::SubAssign for $type<L> {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl<L: LegMarker> core::ops::Mul for $type<L> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                $type(self.0 * rhs.0, core::marker::PhantomData)
            }
        }

        impl<L: LegMarker> core::ops::Div for $type<L> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                $type(self.0 / rhs.0, core::marker::PhantomData)
            }
        }

        impl<L: LegMarker> $type<L> {
            pub const ZERO: Self = $type(0 as $t, core::marker::PhantomData);
            pub const MAX: Self = $type(<$t>::MAX, core::marker::PhantomData);

            pub fn new(value: $t) -> Self {
                $type(value, core::marker::PhantomData)
            }

            pub fn min(self, other: Self) -> Self {
                if self.0 < other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn max(self, other: Self) -> Self {
                if self.0 > other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn checked_add(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_add(rhs.0)
                    .map(|v| $type(v, core::marker::PhantomData))
                    .ok_or(crate::goblin_error::GoblinError::Overflow)
            }

            pub fn checked_sub(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_sub(rhs.0)
                    .map(|v| $type(v, core::marker::PhantomData))
                    .ok_or(crate::goblin_error::GoblinError::Underflow)
            }
        }
    };
}

#[macro_export]
macro_rules! define_dimensionless_type {
    ($type:ident<$t:ty>) => {
        #[repr(C)]
        #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $type(pub $t);

        impl From<$t> for $type {
            fn from(value: $t) -> Self {
                $type(value)
            }
        }

        impl core::ops::Add for $type {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $type(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl core::ops::Sub for $type {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $type(self.0 - rhs.0)
            }
        }

        impl core::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl core::ops::Mul for $type {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                $type(self.0 * rhs.0)
            }
        }

        impl core::ops::Div for $type {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                $type(self.0 / rhs.0)
            }
        }

        impl $type {
            pub const ZERO: Self = $type(0 as $t);
            pub const MAX: Self = $type(<$t>::MAX);

            pub fn min(self, other: Self) -> Self {
                if self.0 < other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn max(self, other: Self) -> Self {
                if self.0 > other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn checked_add(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_add(rhs.0)
                    .map($type)
                    .ok_or(crate::goblin_error::GoblinError::Overflow)
            }

            pub fn checked_sub(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_sub(rhs.0)
                    .map($type)
                    .ok_or(crate::goblin_error::GoblinError::Underflow)
            }
        }
    };
}
#[macro_export]
macro_rules! define_ratio_type {
    ($numerator:ty, $num_type:ty, $denominator:ty, $den_type:ty) => {
        impl From<u64> for Ratio<$numerator, $denominator> {
            fn from(value: u64) -> Self {
                Ratio(value, core::marker::PhantomData)
            }
        }

        impl core::ops::Add for Ratio<$numerator, $denominator> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Ratio(self.0 + rhs.0, core::marker::PhantomData)
            }
        }

        impl core::ops::AddAssign for Ratio<$numerator, $denominator> {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl core::ops::Sub for Ratio<$numerator, $denominator> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Ratio(self.0 - rhs.0, core::marker::PhantomData)
            }
        }

        impl core::ops::SubAssign for Ratio<$numerator, $denominator> {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl core::ops::Mul for Ratio<$numerator, $denominator> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Ratio(self.0 * rhs.0, core::marker::PhantomData)
            }
        }

        impl core::ops::Div for Ratio<$numerator, $denominator> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Ratio(self.0 / rhs.0, core::marker::PhantomData)
            }
        }

        impl Ratio<$numerator, $denominator> {
            pub const ZERO: Self = Ratio(0u64, core::marker::PhantomData);
            pub const MAX: Self = Ratio(u64::MAX, core::marker::PhantomData);

            pub fn new(value: u64) -> Self {
                Ratio(value, core::marker::PhantomData)
            }

            pub fn min(self, other: Self) -> Self {
                if self.0 < other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn max(self, other: Self) -> Self {
                if self.0 > other.0 {
                    self
                } else {
                    other
                }
            }

            pub fn checked_add(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_add(rhs.0)
                    .map(|v| Ratio(v, core::marker::PhantomData))
                    .ok_or(crate::goblin_error::GoblinError::Overflow)
            }

            pub fn checked_sub(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                self.0
                    .checked_sub(rhs.0)
                    .map(|v| Ratio(v, core::marker::PhantomData))
                    .ok_or(crate::goblin_error::GoblinError::Underflow)
            }
        }

        // Core library trait implementations for Ratio operations

        // Division: numerator / denominator = ratio
        impl core::ops::Div<$denominator> for $numerator {
            type Output = Ratio<$numerator, $denominator>;

            fn div(self, rhs: $denominator) -> Self::Output {
                Ratio((self.0 as u64) / (rhs.0 as u64), core::marker::PhantomData)
            }
        }

        // Multiplication: ratio * denominator = numerator
        impl core::ops::Mul<$denominator> for Ratio<$numerator, $denominator> {
            type Output = $numerator;

            fn mul(self, rhs: $denominator) -> Self::Output {
                <$numerator>::from((self.0 * (rhs.0 as u64)) as $num_type)
            }
        }

        // Division: numerator / ratio = denominator
        impl core::ops::Div<Ratio<$numerator, $denominator>> for $numerator {
            type Output = $denominator;

            fn div(self, rhs: Ratio<$numerator, $denominator>) -> Self::Output {
                <$denominator>::from(((self.0 as u64) / rhs.0) as $den_type)
            }
        }
    };
}
