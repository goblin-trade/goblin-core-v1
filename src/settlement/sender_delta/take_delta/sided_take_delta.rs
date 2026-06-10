use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath, leg_quantities::LegQuantities},
        market::LotSizePair,
    },
    quantities::{AsUnsided, BaseLotsPerBaseUnit},
    settlement::{sender_delta::TakeDelta, IntoUnsided, UnsidedTakeDeltaV2},
};

pub type SidedTakeDeltaV2<In: LegMath> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

impl<In> IntoUnsided<In> for SidedTakeDeltaV2<In>
where
    In: LegMatcher,
{
    type Unsided = UnsidedTakeDeltaV2;

    fn into_insided(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        UnsidedTakeDeltaV2 {
            take_in: In::matching_lots_to_unsided_atoms(self.take_in, lot_size_pair),
            take_out: <<In as LegMatcher>::Opposite as LegMatcher>::matching_lots_to_unsided_atoms(
                self.take_out,
                lot_size_pair,
            ),
        }
    }
}

// impl<In> SidedTakeDeltaV2<In>
// where
//     In: LegMath,
// {
//     pub fn into_unsided(
//         &self,
//         base_lot_size: BaseLotsPerBaseUnit,
//         atoms_per_lot: In::AtomsPerLot,
//         opposite_atoms_per_lot: <<In as LegMath>::Opposite as LegQuantities>::AtomsPerLot,
//     ) -> UnsidedTakeDeltaV2 {
//         UnsidedTakeDeltaV2 {
//             take_in: (In::decode_matching_lots(self.take_in, base_lot_size) * atoms_per_lot)
//                 .unsided(),
//             take_out: (<<In as LegMath>::Opposite as LegMath>::decode_matching_lots(
//                 self.take_out,
//                 base_lot_size,
//             ) * opposite_atoms_per_lot)
//                 .unsided(),
//         }
//     }
// }
