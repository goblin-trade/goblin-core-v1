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
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio<N, D>(pub u64, pub core::marker::PhantomData<(N, D)>);

// Define ratio implementations for all leg combinations
define_ratio_type!(Lots<Base>, Units<Base>);
define_ratio_type!(Lots<Quote>, Units<Quote>);
define_ratio_type!(Lots<Quote>, Units<Base>);

#[cfg(test)]
mod tests {

    use core::ops::Div;

    use super::*;
    #[test]
    fn test_ratio() {
        let base_lots = Lots::<Base>::from(1);
        let base_unit = Units::<Base>::from(1);

        let quote_lots = Lots::<Quote>::from(1);
        let quote_unit = Units::<Quote>::from(1);

        let ratio_0 = Ratio::<Lots<Base>, Units<Base>>::from_division(base_lots, base_unit);
        let ratio_1 = Ratio::<Lots<Quote>, Units<Quote>>::from_division(quote_lots, quote_unit);

        // let ratio_3 = Ratio::<Lots<Quote>, Units<Base>>::from_division(quote_lots, base_unit);
    }
}

// pub struct Dim<const TICKS: i8, const BASE_LOTS: i8, const QUOTE_LOTS: i8>;

// pub struct Quantity<V, D> {
//     pub value: V,
//     _phantom: PhantomData<D>,
// }

// // Basic quantity types with leg markers
// define_legged_type!(AtomsV2<u64>);
// define_legged_type!(LotsV2<u64>);

// // The only dimensionless unit
// define_dimensionless_type!(TicksV2<u32>);
