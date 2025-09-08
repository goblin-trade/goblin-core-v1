// macros.rs

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

#[macro_export]
macro_rules! define_inter_type_operations {
    ($type_1:ident<$t1:ty>, $type_2:ident<$t2:ty>, $type_result:ident<$tr:ty>) => {
        // type_1 * type_2 = type_result
        impl core::ops::Mul<$type_2> for $type_1 {
            type Output = $type_result;

            fn mul(self, rhs: $type_2) -> Self::Output {
                $type_result(self.0 as $tr * rhs.0 as $tr)
            }
        }

        // type_2 * type_1 = type_result
        impl core::ops::Mul<$type_1> for $type_2 {
            type Output = $type_result;

            fn mul(self, rhs: $type_1) -> Self::Output {
                $type_result(self.0 as $tr * rhs.0 as $tr)
            }
        }

        // type_result / type_2 = type_1
        impl core::ops::Div<$type_2> for $type_result {
            type Output = $type_1;

            fn div(self, rhs: $type_2) -> Self::Output {
                $type_1((self.0 as $tr / rhs.0 as $tr) as $t1)
            }
        }

        // type_result / type_1 = type_2
        impl core::ops::Div<$type_1> for $type_result {
            type Output = $type_2;

            fn div(self, rhs: $type_1) -> Self::Output {
                $type_2((self.0 as $tr / rhs.0 as $tr) as $t2)
            }
        }

        // type_result % type_2 = type_1
        impl core::ops::Rem<$type_2> for $type_result {
            type Output = $type_1;

            fn rem(self, rhs: $type_2) -> Self::Output {
                $type_1((self.0 as $tr % rhs.0 as $tr) as $t1)
            }
        }

        // type_result % type_1 = type_2
        impl core::ops::Rem<$type_1> for $type_result {
            type Output = $type_2;

            fn rem(self, rhs: $type_1) -> Self::Output {
                $type_2((self.0 as $tr % rhs.0 as $tr) as $t2)
            }
        }
    };
}

#[macro_export]
macro_rules! define_delta_operations {
    ($delta_type:ident<$delta_inner:ty>, $base_type:ident<$base_inner:ty>) => {
        // DeltaType + BaseType = Result<DeltaType, GoblinError>
        impl core::ops::Add<$base_type> for $delta_type {
            type Output = Result<$delta_type, crate::goblin_error::GoblinError>;

            fn add(self, base: $base_type) -> Self::Output {
                // Ensure the base value fits in delta's inner type
                if let Ok(val) = <$delta_inner>::try_from(base.0) {
                    self.0
                        .checked_add(val)
                        .map($delta_type)
                        .ok_or(crate::goblin_error::GoblinError::DeltaOverflow)
                } else {
                    Err(crate::goblin_error::GoblinError::DeltaOverflow)
                }
            }
        }

        // DeltaType - BaseType = Result<DeltaType, GoblinError>
        impl core::ops::Sub<$base_type> for $delta_type {
            type Output = Result<$delta_type, crate::goblin_error::GoblinError>;

            fn sub(self, base: $base_type) -> Self::Output {
                if let Ok(val) = <$delta_inner>::try_from(base.0) {
                    self.0
                        .checked_sub(val)
                        .map($delta_type)
                        .ok_or(crate::goblin_error::GoblinError::DeltaUnderflow)
                } else {
                    Err(crate::goblin_error::GoblinError::DeltaUnderflow)
                }
            }
        }

        // BaseType + DeltaType = Result<BaseType, GoblinError>
        impl core::ops::Add<$delta_type> for $base_type {
            type Output = Result<$base_type, crate::goblin_error::GoblinError>;

            fn add(self, delta: $delta_type) -> Self::Output {
                if delta.0 >= 0 {
                    self.0
                        .checked_add(delta.0 as $base_inner)
                        .map($base_type)
                        .ok_or(crate::goblin_error::GoblinError::Overflow)
                } else {
                    self.0
                        .checked_sub(delta.0.unsigned_abs())
                        .map($base_type)
                        .ok_or(crate::goblin_error::GoblinError::Underflow)
                }
            }
        }

        // BaseType - DeltaType = Result<BaseType, GoblinError>
        impl core::ops::Sub<$delta_type> for $base_type {
            type Output = Result<$base_type, crate::goblin_error::GoblinError>;

            fn sub(self, delta: $delta_type) -> Self::Output {
                if delta.0 >= 0 {
                    self.0
                        .checked_sub(delta.0 as $base_inner)
                        .map($base_type)
                        .ok_or(crate::goblin_error::GoblinError::Underflow)
                } else {
                    self.0
                        .checked_add(delta.0.unsigned_abs())
                        .map($base_type)
                        .ok_or(crate::goblin_error::GoblinError::Overflow)
                }
            }
        }
    };
}
