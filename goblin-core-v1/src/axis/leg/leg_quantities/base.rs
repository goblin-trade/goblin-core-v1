use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base},
    quantities::{
        BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseDeltaLots, BaseLots,
        BaseLotsPerBaseUnit, BaseUnits,
    },
};

impl LegQuantities for Base {
    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type DeltaLots = BaseDeltaLots;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;
}
