use core::marker::PhantomData;

use crate::types::{leg::LegMarker, Base, Quote};

// 1. Base units

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tick(pub u64);

pub struct Lots<L: LegMarker>(pub u64, PhantomData<L>);
pub struct Units<L: LegMarker>(pub u64, PhantomData<L>);
pub struct Atom<L: LegMarker>(pub u64, PhantomData<L>);

// 2. Composite traits

pub struct Ratio<N, D>(pub u64, PhantomData<(N, D)>);
pub struct Prod<A, B>(pub u64, PhantomData<(A, B)>);

// Need operations between these 3 types
pub type BaseLotsPerBaseUnitV2 = Ratio<Lots<Base>, Units<Base>>;

// // 2. Powered unit
// //
// // Allows us to use a single Prod<> struct instead of Prod<> and Ratio<>
// //
// // # Problem
// // We only have +1 and -1 power, so unnecesary
// pub struct PoweredLot<L: LegMarker, const P: i8>(pub u64, PhantomData<L>);
// pub struct PoweredUnit<L: LegMarker, const P: i8>(pub u64, PhantomData<L>);
// pub struct PoweredAtoms<L: LegMarker, const P: i8>(pub u64, PhantomData<L>);

// // 3. Dimenstion algebra
// //
// // # Problems
// // - Can't fit Ticks as u32
// // - Can't use `LegMarker`
// pub struct SidedVal<M: LegMarker, const L: i8, const U: i8, const A: i8>(pub u64, PhantomData<M>);

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_ratio() {
        let base_lots = Lots::<Base>(1, PhantomData::<Base>);
        let base_unit = Units::<Base>(1, PhantomData::<Base>);

        // let rat = base_lots / base_unit;
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
