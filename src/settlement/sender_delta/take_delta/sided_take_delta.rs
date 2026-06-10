use crate::{
    axis::leg::{leg_math::LegMath, leg_quantities::LegQuantities},
    quantities::{AsUnsided, BaseLotsPerBaseUnit},
    settlement::{sender_delta::TakeDelta, UnsidedTakeDeltaV2},
};

pub type SidedTakeDeltaV2<In: LegMath> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

impl<In> SidedTakeDeltaV2<In>
where
    In: LegMath,
{
    pub fn into_unsided(
        &self,
        base_lot_size: BaseLotsPerBaseUnit,
        atoms_per_lot: In::AtomsPerLot,
        opposite_atoms_per_lot: <<In as LegMath>::Opposite as LegQuantities>::AtomsPerLot,
    ) -> UnsidedTakeDeltaV2 {
        UnsidedTakeDeltaV2 {
            take_in: (In::decode_matching_lots(self.take_in, base_lot_size) * atoms_per_lot)
                .unsided(),
            take_out: (<<In as LegMath>::Opposite as LegMath>::decode_matching_lots(
                self.take_out,
                base_lot_size,
            ) * opposite_atoms_per_lot)
                .unsided(),
        }
    }
}
