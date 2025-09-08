#[macro_export]
macro_rules! define_custom_types_v2 {
    ($($type:ident<L: LegMarker, $t:ty>),*) => {
        $(
            #[repr(C)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
            pub struct $type<L: crate::types::leg::LegMarker>(pub $t, core::marker::PhantomData<L>);

            impl<L: crate::types::leg::LegMarker> From<$t> for $type<L> {
                fn from(value: $t) -> Self {
                    $type(value, core::marker::PhantomData)
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::Add for $type<L> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    $type(self.0 + rhs.0, core::marker::PhantomData)
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::AddAssign for $type<L> {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0;
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::Sub for $type<L> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    $type(self.0 - rhs.0, core::marker::PhantomData)
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::SubAssign for $type<L> {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0;
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::Mul for $type<L> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    $type(self.0 * rhs.0, core::marker::PhantomData)
                }
            }

            impl<L: crate::types::leg::LegMarker> core::ops::Div for $type<L> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    $type(self.0 / rhs.0, core::marker::PhantomData)
                }
            }

            impl<L: crate::types::leg::LegMarker> $type<L> {
                pub const ZERO: Self = $type(0 as $t, core::marker::PhantomData);
                pub const MAX: Self = $type(<$t>::MAX, core::marker::PhantomData);

                pub fn new(value: $t) -> Self {
                    $type(value, core::marker::PhantomData)
                }

                pub fn value(&self) -> $t {
                    self.0
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
                    self.0.checked_add(rhs.0)
                        .map(|val| $type(val, core::marker::PhantomData))
                        .ok_or(crate::goblin_error::GoblinError::Overflow)
                }

                pub fn checked_sub(self, rhs: Self) -> Result<Self, crate::goblin_error::GoblinError> {
                    self.0.checked_sub(rhs.0)
                        .map(|val| $type(val, core::marker::PhantomData))
                        .ok_or(crate::goblin_error::GoblinError::Underflow)
                }
            }
        )*
    };
}
