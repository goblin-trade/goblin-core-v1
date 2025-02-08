//! We define wrapper types around primitive number types to ensure that we
//! only do arithmetic on quantities that make sense.

use borsh::{BorshDeserialize as Deserialize, BorshSerialize as Serialize};
use bytemuck::{Pod, Zeroable};
use core::fmt::Display;
use core::iter::Sum;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};
use stylus_sdk::alloy_primitives::U256;

use stylus_sdk::console;

use crate::parameters::{
    BASE_DECIMALS_TO_IGNORE, BASE_LOT_SIZE, MAX_RAW_BASE_ATOMS, MAX_RAW_QUOTE_ATOMS,
    QUOTE_DECIMALS_TO_IGNORE, QUOTE_LOT_SIZE,
};
use crate::program::{ExceedTickSize, GoblinError, GoblinResult, InvalidFeeCollector};
use crate::require;
use crate::state::Side;

pub trait WrapperU64 {
    fn new(value: u64) -> Self;
    fn as_u64(&self) -> u64;
}

pub trait WrapperLimitedU64<E> {
    fn new(value: u64) -> Result<Self, E>
    where
        Self: Sized;
    fn as_u64(&self) -> u64;
}

macro_rules! basic_u64_struct {
    ($type_name:ident) => {
        #[derive(Debug, Clone, Copy, PartialOrd, Ord, Zeroable, Pod)]
        #[repr(transparent)]
        pub struct $type_name {
            pub inner: u64,
        }

        basic_u64!($type_name, 64);
    };
}

macro_rules! basic_u64 {
    ($type_name:ident, $max_bits:expr) => {
        impl WrapperU64 for $type_name {
            fn new(value: u64) -> Self {
                assert!($max_bits <= 64);

                if $max_bits < 64 {
                    assert!(
                        value < (1 << $max_bits),
                        "Value exceeds maximum allowed bits"
                    );
                }

                $type_name { inner: value }
            }

            fn as_u64(&self) -> u64 {
                self.inner
            }
        }

        impl $type_name {
            pub const ZERO: Self = $type_name { inner: 0 };
            pub const ONE: Self = $type_name { inner: 1 };
            // pub const MAX: Self = $type_name { inner: u64::MAX };
            pub const MAX: Self = $type_name {
                inner: ((1u128 << $max_bits) - 1) as u64,
            };

            pub const MIN: Self = $type_name { inner: u64::MIN };
            pub fn as_u128(&self) -> u128 {
                self.inner as u128
            }

            pub fn saturating_sub(self, other: Self) -> Self {
                $type_name::new(self.inner.saturating_sub(other.inner))
            }

            pub fn unchecked_div<Divisor: WrapperU64, Quotient: WrapperU64>(
                self,
                other: Divisor,
            ) -> Quotient {
                Quotient::new(self.inner / other.as_u64())
            }

            pub fn div_ceil<Divisor: WrapperU64, Quotient: WrapperU64>(
                self,
                other: Divisor,
            ) -> Quotient {
                Quotient::new(self.inner.div_ceil(other.as_u64()))
            }
        }

        // TODO remove
        impl Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.inner.fmt(f)
            }
        }

        impl Mul for $type_name {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                $type_name::new(self.inner * other.inner)
            }
        }

        impl Sum<$type_name> for $type_name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold($type_name::ZERO, |acc, x| acc + x)
            }
        }

        impl Add for $type_name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $type_name::new(self.inner + other.inner)
            }
        }

        impl AddAssign for $type_name {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl Sub for $type_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                $type_name::new(self.inner - other.inner)
            }
        }

        impl SubAssign for $type_name {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }

        impl Default for $type_name {
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl PartialEq for $type_name {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }

        impl From<$type_name> for u64 {
            fn from(x: $type_name) -> u64 {
                x.inner
            }
        }

        impl From<$type_name> for f64 {
            fn from(x: $type_name) -> f64 {
                x.inner as f64
            }
        }

        impl Eq for $type_name {}

        // Below should only be used in tests.
        impl PartialEq<u64> for $type_name {
            fn eq(&self, other: &u64) -> bool {
                self.inner == *other
            }
        }

        impl PartialEq<$type_name> for u64 {
            fn eq(&self, other: &$type_name) -> bool {
                *self == other.inner
            }
        }
    };
}

// GoblinResult;

// macro_rules! limited_size_u64 {
//     ($type_name:ident, $max_bits:expr, $error:expr) => {
//         impl WrapperLimitedU64<$error> for $type_name {
//             fn new(value: u64) -> Result<Self, $error> {
//                 assert!($max_bits <= 64);

//                 if $max_bits < 64 {
//                     require!(
//                         value < (1 << $max_bits),
//                         $error
//                     );
//                 }

//                 Ok($type_name { inner: value })

//                 // $type_name { inner: value }
//             }

//             fn as_u64(&self) -> u64 {
//                 self.inner
//             }
//         }

//         impl $type_name {
//             pub const ZERO: Self = $type_name { inner: 0 };
//             pub const ONE: Self = $type_name { inner: 1 };
//             // pub const MAX: Self = $type_name { inner: u64::MAX };
//             pub const MAX: Self = $type_name {
//                 inner: ((1u128 << $max_bits) - 1) as u64,
//             };

//             pub const MIN: Self = $type_name { inner: u64::MIN };
//             pub fn as_u128(&self) -> u128 {
//                 self.inner as u128
//             }

//             pub fn saturating_sub(self, other: Self) -> Self {
//                 $type_name::new(self.inner.saturating_sub(other.inner))
//             }

//             pub fn unchecked_div<Divisor: WrapperU64, Quotient: WrapperU64>(
//                 self,
//                 other: Divisor,
//             ) -> Quotient {
//                 Quotient::new(self.inner / other.as_u64())
//             }
//         }

//         // TODO remove
//         impl Display for $type_name {
//             fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//                 self.inner.fmt(f)
//             }
//         }

//         impl Mul for $type_name {
//             type Output = Self;
//             fn mul(self, other: Self) -> Self {
//                 $type_name::new(self.inner * other.inner)
//             }
//         }

//         impl Sum<$type_name> for $type_name {
//             fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//                 iter.fold($type_name::ZERO, |acc, x| acc + x)
//             }
//         }

//         impl Add for $type_name {
//             type Output = Self;
//             fn add(self, other: Self) -> Self {
//                 $type_name::new(self.inner + other.inner)
//             }
//         }

//         impl AddAssign for $type_name {
//             fn add_assign(&mut self, other: Self) {
//                 *self = *self + other;
//             }
//         }

//         impl Sub for $type_name {
//             type Output = Self;

//             fn sub(self, other: Self) -> Self {
//                 $type_name::new(self.inner - other.inner)
//             }
//         }

//         impl SubAssign for $type_name {
//             fn sub_assign(&mut self, other: Self) {
//                 *self = *self - other;
//             }
//         }

//         impl Default for $type_name {
//             fn default() -> Self {
//                 Self::ZERO
//             }
//         }

//         impl PartialEq for $type_name {
//             fn eq(&self, other: &Self) -> bool {
//                 self.inner == other.inner
//             }
//         }

//         impl From<$type_name> for u64 {
//             fn from(x: $type_name) -> u64 {
//                 x.inner
//             }
//         }

//         impl From<$type_name> for f64 {
//             fn from(x: $type_name) -> f64 {
//                 x.inner as f64
//             }
//         }

//         impl Eq for $type_name {}

//         // Below should only be used in tests.
//         impl PartialEq<u64> for $type_name {
//             fn eq(&self, other: &u64) -> bool {
//                 self.inner == *other
//             }
//         }

//         impl PartialEq<$type_name> for u64 {
//             fn eq(&self, other: &$type_name) -> bool {
//                 *self == other.inner
//             }
//         }
//     };
// }

macro_rules! allow_multiply {
    ($type_1:ident, $type_2:ident, $type_result:ident) => {
        impl Mul<$type_2> for $type_1 {
            type Output = $type_result;
            fn mul(self, other: $type_2) -> $type_result {
                $type_result::new(self.inner * other.inner)
            }
        }

        impl Mul<$type_1> for $type_2 {
            type Output = $type_result;
            fn mul(self, other: $type_1) -> $type_result {
                $type_result::new(self.inner * other.inner)
            }
        }

        impl Div<$type_1> for $type_result {
            type Output = $type_2;
            #[track_caller]
            fn div(self, other: $type_1) -> $type_2 {
                if self.inner % other.inner != 0 {
                    let caller = core::panic::Location::caller();

                    // console!(
                    //     "WARNING: Expected clean division, but received {:?} / {:?}. Caller: {:?}",
                    //     self,
                    //     other,
                    //     caller
                    // );
                }
                $type_2::new(self.inner / other.inner)
            }
        }

        impl Div<$type_2> for $type_result {
            type Output = $type_1;
            #[track_caller]
            fn div(self, other: $type_2) -> $type_1 {
                if self.inner % other.inner != 0 {
                    let caller = core::panic::Location::caller();

                    // console!(
                    //     "WARNING: Expected clean division, but received {:?} / {:?}. Caller: {:?}",
                    //     self,
                    //     other,
                    //     caller
                    // );
                }
                $type_1::new(self.inner / other.inner)
            }
        }

        // impl $type_result {
        //     /// Division with ceiling for cases where the result is not an integer.
        //     #[track_caller]
        //     pub fn div_ceil(self, other: $type_1) -> $type_2 {
        //         let result = (self.inner + other.inner - 1) / other.inner;
        //         $type_2::new(result)
        //     }

        //     // /// Division with ceiling for cases where the result is not an integer.
        //     // #[track_caller]
        //     // pub fn div_ceil(self, other: $type_2) -> $type_1 {
        //     //     let result = (self.inner + other.inner - 1) / other.inner;
        //     //     $type_1::new(result)
        //     // }
        // }
    };
}

macro_rules! allow_mod {
    ($type_1:ident, $type_2:ident) => {
        impl Rem<$type_2> for $type_1 {
            type Output = u64;
            fn rem(self, other: $type_2) -> u64 {
                self.inner % other.inner
            }
        }
    };
}

// These structs need to be explicitly defined outside of the macro generation because the
// OrderPacket type (which contains these units) implements BorshSerialize and BorshDeserialize
#[derive(Debug, Clone, Copy, PartialOrd, Ord, Zeroable, Pod, Deserialize, Serialize)]
#[repr(transparent)]
pub struct QuoteLots {
    inner: u64,
}
#[derive(Debug, Clone, Copy, PartialOrd, Ord, Zeroable, Pod, Deserialize, Serialize)]
#[repr(transparent)]
pub struct BaseLots {
    inner: u64,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Zeroable, Pod, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Ticks {
    inner: u64,
}

// Discrete price unit (quote quantity per base quantity)
// 16 bits from bitmap index, 5 bits from resting order index
// Ticks can be fit within u32, but we're using u64 for convenience
basic_u64!(Ticks, 21); // TODO impose limit elsewhere

pub const MAX_TICK: u32 = 2097151; // 2^21 - 1

impl Ticks {
    pub const BID_DEFAULT: Self = Ticks::MIN;
    pub const ASK_DEFAULT: Self = Ticks::MAX; // TODO should be equal to MAX_TICK

    pub fn default_for_side(side: Side) -> Self {
        match side {
            Side::Bid => Ticks::BID_DEFAULT,
            Side::Ask => Ticks::ASK_DEFAULT,
        }
    }

    pub fn try_encode(value: u32) -> GoblinResult<Self> {
        require!(
            value <= MAX_TICK,
            GoblinError::ExceedTickSize(ExceedTickSize {})
        );

        Ok(Ticks {
            inner: value as u64,
        })
    }
}

// BaseLots have smaller max size than QuoteLots
basic_u64!(QuoteLots, 64);
basic_u64!(BaseLots, 64); // size limit imposed in RestingOrder::encode()

// Quantities
basic_u64_struct!(QuoteAtoms);
basic_u64_struct!(BaseAtoms);

// basic_u64_struct!(QuoteUnits);
// basic_u64_struct!(BaseUnits);

// Dimensionless conversion factors
basic_u64_struct!(QuoteAtomsPerQuoteLot);
basic_u64_struct!(BaseAtomsPerBaseLot);

basic_u64_struct!(BaseLotsPerBaseUnit);
basic_u64_struct!(QuoteLotsPerQuoteUnit);

basic_u64_struct!(QuoteAtomsPerQuoteUnit);
basic_u64_struct!(BaseAtomsPerBaseUnit);

// Dimensionless tick sizes
basic_u64_struct!(QuoteAtomsPerBaseUnitPerTick);
basic_u64_struct!(QuoteLotsPerBaseUnitPerTick);

basic_u64_struct!(AdjustedQuoteLots);
basic_u64_struct!(QuoteLotsPerBaseUnit);

// Conversions from units to lots
// allow_multiply!(BaseUnits, BaseLotsPerBaseUnit, BaseLots);
// allow_multiply!(QuoteUnits, QuoteLotsPerQuoteUnit, QuoteLots);

// Conversions from lots to atoms
allow_multiply!(QuoteLots, QuoteAtomsPerQuoteLot, QuoteAtoms);
allow_multiply!(BaseLots, BaseAtomsPerBaseLot, BaseAtoms);

// Conversion from atoms per lot to units
// allow_multiply!(
//     BaseAtomsPerBaseLot,
//     BaseLotsPerBaseUnit,
//     BaseAtomsPerBaseUnit
// );
// allow_multiply!(
//     QuoteAtomsPerQuoteLot,
//     QuoteLotsPerQuoteUnit,
//     QuoteAtomsPerQuoteUnit
// );

// Conversion between units of tick size
// allow_multiply!(
//     QuoteLotsPerBaseUnitPerTick,
//     QuoteAtomsPerQuoteLot,
//     QuoteAtomsPerBaseUnitPerTick
// );

// Conversion from ticks to price
allow_multiply!(QuoteLotsPerBaseUnitPerTick, Ticks, QuoteLotsPerBaseUnit);

// Conversion from quote lots to adjusted quote lots
allow_multiply!(QuoteLots, BaseLotsPerBaseUnit, AdjustedQuoteLots);

// Intermediate conversions for extracting quote lots from book orders
allow_multiply!(QuoteLotsPerBaseUnit, BaseLots, AdjustedQuoteLots);

// allow_mod!(AdjustedQuoteLots, BaseLotsPerBaseUnit);
// allow_mod!(BaseAtomsPerBaseUnit, BaseLotsPerBaseUnit);
// allow_mod!(QuoteAtomsPerQuoteUnit, QuoteLotsPerQuoteUnit);
// allow_mod!(QuoteLotsPerBaseUnitPerTick, BaseLotsPerBaseUnit);

pub struct QuoteAtomsRaw {
    inner: U256,
}

pub struct BaseAtomsRaw {
    inner: U256,
}

impl QuoteAtomsRaw {
    pub const MAX: Self = QuoteAtomsRaw {
        inner: U256::from_limbs(MAX_RAW_QUOTE_ATOMS),
    };

    pub fn from_u256(value: U256) -> Self {
        // capped to MAX_RAW_QUOTE_ATOMS
        if value > QuoteAtomsRaw::MAX.as_u256() {
            QuoteAtomsRaw::MAX
        } else {
            QuoteAtomsRaw { inner: value }
        }
    }

    pub fn as_u256(&self) -> U256 {
        self.inner
    }

    pub fn from_lots(lots: QuoteLots) -> Self {
        // Edge case- if lots are so large such that raw atoms exceed QuoteAtomsRaw::MAX,
        // they are capped to QuoteAtomsRaw::MAX
        QuoteAtomsRaw::from_u256(
            U256::from(lots.as_u64())
                * U256::from(QUOTE_LOT_SIZE.as_u64()) // lots to atoms
                * U256::from_limbs(QUOTE_DECIMALS_TO_IGNORE), // atoms to raw atoms
        )
    }

    pub fn to_atoms(&self) -> QuoteAtoms {
        // MAX value ensures that division by QUOTE_DECIMALS_TO_IGNORE fits in u64
        QuoteAtoms::new((self.inner / U256::from_limbs(QUOTE_DECIMALS_TO_IGNORE)).as_limbs()[0])
    }

    pub fn to_lots(&self) -> QuoteLots {
        self.to_atoms() / QUOTE_LOT_SIZE
    }
}

// BaseAtomsRaw: U256 can be a large value. We must ensure that BaseAtoms obtained after division
// with BASE_DECIMALS_TO_IGNORE fits in u64
impl BaseAtomsRaw {
    pub const MAX: Self = BaseAtomsRaw {
        inner: U256::from_limbs(MAX_RAW_BASE_ATOMS),
    };

    pub fn from_u256(value: U256) -> Self {
        // capped to MAX_RAW_BASE_ATOMS
        if value > BaseAtomsRaw::MAX.as_u256() {
            BaseAtomsRaw::MAX
        } else {
            BaseAtomsRaw { inner: value }
        }
    }

    pub fn as_u256(&self) -> U256 {
        self.inner
    }

    pub fn from_lots(lots: BaseLots) -> Self {
        // Edge case- if lots are so large such that raw atoms exceed BaseAtomsRaw::MAX,
        // they are capped to BaseAtomsRaw::MAX
        BaseAtomsRaw::from_u256(
            U256::from(lots.as_u64())
                * U256::from(BASE_LOT_SIZE.as_u64()) // lots to atoms
                * U256::from_limbs(BASE_DECIMALS_TO_IGNORE), // atoms to raw atoms
        )
    }

    pub fn to_atoms(&self) -> BaseAtoms {
        // MAX value ensures that division by BASE_DECIMALS_TO_IGNORE fits in u64
        BaseAtoms::new((self.inner / U256::from_limbs(BASE_DECIMALS_TO_IGNORE)).as_limbs()[0])
    }

    pub fn to_lots(&self) -> BaseLots {
        self.to_atoms() / BASE_LOT_SIZE
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_new_constructor_macro() {
//         let base_lots_1 = BaseLots::new(5);
//         let base_lots_2 = BaseLots::new(10);

//         assert_eq!(base_lots_1 + base_lots_2, BaseLots::new(15));

//         // Below code (correctly) fails to compile.
//         // let quote_lots_1 = QuoteLots::new(5);
//         // let result = quote_lots_1 + base_lots_1;
//     }

//     #[test]
//     fn test_multiply_macro() {
//         let base_units = BaseUnits::new(5);
//         let base_lots_per_base_unit = BaseLotsPerBaseUnit::new(100);
//         assert_eq!(base_units * base_lots_per_base_unit, BaseLots::new(500));

//         // Below code (correctly) fails to compile.
//         // let quote_units = QuoteUnits::new(5);
//         // let result = quote_units * base_lots_per_base_unit;
//     }

//     #[test]
//     #[should_panic]
//     fn test_tick_overflow() {
//         Ticks::new(u64::MAX);
//     }

//     #[test]
//     fn test_max_values() {
//         assert_eq!(Ticks::MAX, 2097151); // 2^21 - 1
//                                          // TODO fix
//         assert_eq!(BaseLots::MAX, 4294967295); // 2^32 - 1
//     }

//     #[test]
//     #[should_panic]
//     fn test_tick_addition_overflow() {
//         println!("max tick {:?}", Ticks::MAX);
//         let tick = Ticks::new(2097151);

//         let _added_tick = tick.add(Ticks::new(1));
//     }

//     #[test]
//     #[should_panic]
//     fn test_tick_multiplication_overflow() {
//         let tick = Ticks::new(2097151);

//         let _multiplied_tick = tick.mul(Ticks::new(2));
//     }

//     #[test]
//     fn raw_atoms_to_lots() {
//         let lots = BaseLots::new(10);
//         let raw_atoms = BaseAtomsRaw::from_lots(lots);

//         let raw_atoms_u256 = raw_atoms.as_u256();
//         println!(
//             "raw_atoms_u256 {:?}, limbs {:?}",
//             raw_atoms_u256,
//             raw_atoms_u256.as_limbs()
//         );

//         let decoded_lots = raw_atoms.to_lots();
//     }

//     #[test]
//     fn get_max_size_for_base() {
//         let atoms = U256::from(u64::MAX);

//         let decimals_to_ignore = U256::from_limbs(BASE_DECIMALS_TO_IGNORE);

//         let max_raw_atoms = atoms * decimals_to_ignore;

//         println!(
//             "max_raw_atoms {:?}, limbs {:?}",
//             max_raw_atoms,
//             max_raw_atoms.as_limbs()
//         );
//     }

//     #[test]
//     fn get_max_size_for_quote() {
//         let atoms = U256::from(u64::MAX);

//         let decimals_to_ignore = U256::from_limbs(QUOTE_DECIMALS_TO_IGNORE);

//         let max_raw_atoms = atoms * decimals_to_ignore;

//         println!(
//             "max_raw_atoms {:?}, limbs {:?}",
//             max_raw_atoms,
//             max_raw_atoms.as_limbs()
//         );
//     }

//     #[test]
//     fn test_max_adusted_quote_lots() {
//         assert_eq!(AdjustedQuoteLots::MAX.as_u64(), u64::MAX);
//     }
// }
