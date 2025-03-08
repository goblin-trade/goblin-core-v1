#[macro_export]
macro_rules! define_custom_types {
    ($($type:ident<$t:ty>),*) => {
        $(
            #[repr(C)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct $type(pub $t);

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
        )*
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
    };
}
