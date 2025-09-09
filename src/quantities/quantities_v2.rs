use core::marker::PhantomData;

use crate::{
    define_dimensionless_type, define_legged_type, define_ratio_type,
    types::{leg::LegMarker, Base, Quote},
};

// 1. Base units

define_dimensionless_type!(Tick<u32>);
define_legged_type!(Atom<u64>);
define_legged_type!(Lots<u64>);
define_legged_type!(Units<u64>);

// 2. Composite units
// Define the generic Ratio struct
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio<N, D>(pub u64, pub core::marker::PhantomData<(N, D)>);

// Define ratio implementations for all leg combinations
define_ratio_type!(Lots<Base>, Units<Base>);
define_ratio_type!(Lots<Quote>, Units<Quote>);
define_ratio_type!(Lots<Quote>, Units<Base>);

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_ratio() {
        let base_lots = Lots::<Base>::from(100);
        let base_unit = Units::<Base>::from(10);

        let quote_lots = Lots::<Quote>::from(200);
        let quote_unit = Units::<Quote>::from(20);

        // Create ratios using division operator
        let ratio_0: Ratio<Lots<Base>, Units<Base>> = base_lots / base_unit;
        let ratio_1: Ratio<Lots<Quote>, Units<Quote>> = quote_lots / quote_unit;
        let ratio_3 = quote_lots / base_unit;

        let base_unit_derived = quote_lots / ratio_3;
        let quote_lots_derived = ratio_3 * base_unit;
    }
}
