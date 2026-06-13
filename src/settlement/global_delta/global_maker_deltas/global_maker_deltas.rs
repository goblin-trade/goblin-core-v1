use crate::{
    axis::token::{
        token_marker::{
            custom_erc20::custom_erc20_index::CustomERC20Index,
            hardcoded_erc20::hardcoded_erc20_index::HardcodedERC20Index,
        },
        Token,
    },
    settlement::{
        global_delta::{
            maker_custom_deltas::MakerCustomDeltas, maker_hardcoded_deltas::MakerHardcodedDeltas,
            ETHMakerDeltas, MakerDeltaKey, UnsidedMakerDelta,
        },
        ConstZero,
    },
    types::{FixedMap, Triple},
};

/// Global deltas of makers that matched against msg.sender
pub type GlobalMakerDeltas = Triple<ETHMakerDeltas, MakerHardcodedDeltas, MakerCustomDeltas, Token>;

impl ConstZero for GlobalMakerDeltas {
    const ZEROED: Self = Self::new(
        ETHMakerDeltas::zero(),
        FixedMap {
            entries: [(
                MakerDeltaKey {
                    maker: [0u8; 20],
                    token_index: HardcodedERC20Index(0),
                },
                UnsidedMakerDelta::ZEROED,
            ); 16],
            len: 0,
        },
        FixedMap {
            entries: [(
                MakerDeltaKey {
                    maker: [0u8; 20],
                    token_index: CustomERC20Index(0),
                },
                UnsidedMakerDelta::ZEROED,
            ); 16],
            len: 0,
        },
    );
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
