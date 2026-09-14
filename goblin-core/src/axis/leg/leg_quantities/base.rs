use crate::{
    axis::leg::{Base, leg_quantities::LegQuantities},
    quantities::{
        BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsPerBaseUnit,
        BaseUnits,
    },
};

impl LegQuantities for Base {
    type Lots = BaseLots<u64>;
    type Units = BaseUnits;
    type Atoms = BaseAtoms<u64>;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;
}
