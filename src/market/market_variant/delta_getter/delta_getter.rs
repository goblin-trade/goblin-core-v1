// use crate::{
//     market::MarketVariant,
//     settlement::global_delta::{
//         ERC20Delta, ERC20MakerDeltas, ERC20SenderDeltas, UnsidedMakerDelta,
//     },
//     types::Address,
// };

// /// Need 2 getters
// /// * hardcoded and custom
// /// * Replace MarketERC20Index with ERC20Index for hardcoded and custom

// /// Get deltas for a market variant
// /// DeltaGetter is just an offshoot of MarketVariant
// /// I moved out the code for clarity
// ///
// /// Move the function directly on ERC20Index?
// ///
// /// No need, use accessors directly
// ///
// /// HardcodedERC20::get_leg_mut(token_sender_deltas).get_delta_mut(market_erc20_index)
// pub trait DeltaGetter: MarketVariant {
//     fn token_sender_delta_mut(
//         market_erc20_index: Self::MarketERC20Index,
//         token_sender_deltas: &mut ERC20SenderDeltas,
//     ) -> &mut ERC20Delta;

//     fn token_maker_delta_mut(
//         market_erc20_index: Self::MarketERC20Index,
//         maker: Address,
//         token_maker_deltas: &mut ERC20MakerDeltas,
//     ) -> Option<&mut UnsidedMakerDelta>;
// }
