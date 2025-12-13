use crate::{
    settlement::global_delta::{ERC20MakerDeltas, ETHMakerDeltas},
    types::TokenPair,
};

/// Global deltas of makers that matched against msg.sender
pub type GlobalMakerDeltas = TokenPair<ETHMakerDeltas, ERC20MakerDeltas>;

impl GlobalMakerDeltas {
    pub const fn zero() -> Self {
        Self::new(ETHMakerDeltas::zero(), ERC20MakerDeltas::zero())
    }
}

// const GLOBAL_MAKER_DELTA_COUNT: usize = 16;
// pub type GlobalMakerDeltas = FixedMap<MakerTokenPair, MakerUpdate, GLOBAL_MAKER_DELTA_COUNT>;

// #[derive(PartialEq, Clone, Copy)]
// pub struct MakerTokenPair {
//     pub maker: Address,
//     pub token_index: DynamicIndex,
// }

// impl GlobalMakerDeltas {
//     pub fn apply_updates(
//         &mut self,
//         indexed_market: &IndexedMarket,
//         deltas: &MarketMakerDeltas,
//     ) -> Result<(), GoblinError> {
//         for (maker, maker_delta) in deltas.iter() {
//             self.apply_side_update::<Base>(indexed_market, maker, maker_delta)?;
//             self.apply_side_update::<Quote>(indexed_market, maker, maker_delta)?;
//         }

//         Ok(())
//     }

//     pub fn settle(&self) {}

//     fn apply_side_update<In>(
//         &mut self,
//         indexed_market: &IndexedMarket,
//         maker: &Address,
//         maker_delta: &MakerDelta,
//     ) -> Result<(), GoblinError>
//     where
//         In: LegMarker
//             + PairAccessor<DynamicIndex, DynamicIndex, Result = DynamicIndex>
//             + PairAccessor<
//                 <Base as LegMarker>::LotsPerUnit,
//                 <Quote as LegMarker>::LotsPerUnit,
//                 Result = In::LotsPerUnit,
//             > + PairAccessor<MakerSideDelta<Base>, MakerSideDelta<Quote>, Result = MakerSideDelta<In>>,
//         In::Opposite: PairAccessor<
//             MakerSideDelta<Base>,
//             MakerSideDelta<Quote>,
//             Result = MakerSideDelta<In::Opposite>,
//         >,
//     {
//         let update = MakerUpdate::new::<In>(
//             maker_delta,
//             indexed_market.lot_size_pair,
//             indexed_market.base_lot_size(),
//         );

//         let token_index = *In::get_leg(&indexed_market.token_index_pair);

//         // Write to store
//         let store = self
//             .get_or_insert_mut(MakerTokenPair {
//                 maker: *maker,
//                 token_index,
//             })
//             .ok_or(GoblinError::MakerStoreListFull)?;
//         store.free_atoms_in += update.free_atoms_in;
//         store.locked_atoms_out += update.locked_atoms_out;

//         Ok(())
//     }
// }
