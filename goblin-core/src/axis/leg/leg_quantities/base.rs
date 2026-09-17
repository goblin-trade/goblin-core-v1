use crate::{
    axis::leg::{Base, leg_quantities::LegQuantities},
    quantities::{
        BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsPerBaseUnit,
        BaseUnits,
    },
};

impl LegQuantities for Base {
    type Lots = BaseLots<u64>;
    type Units = BaseUnits<u64>;
    type Atoms = BaseAtoms<u64>;

    type LotsPerUnit = BaseLotsPerBaseUnit<u64>;
    type AtomsPerUnit = BaseAtomsPerBaseUnit<u64>;
    type AtomsPerLot = BaseAtomsPerBaseLot<u64>;
}
