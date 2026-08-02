#[macro_export]
macro_rules! define_custom_type {
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
