///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either dynamic or custom
use crate::{
    settlement::global_delta::{
        ERC20Delta, ERC20MakerDeltas, ERC20SenderDeltas, UnsidedMakerDelta,
    },
    state::SlotKey,
    token::TokenMarker,
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

pub trait MarketVariant: Clone + Copy {
    const DISCRIMINATOR: u8;

    type TokenIndex: Clone + Copy;

    type Market<B: TokenMarker, Q: TokenMarker>;

    /// Key to read market state slot
    type MarketKey<B: TokenMarker, Q: TokenMarker>: SlotKey;

    fn token_sender_delta_mut(
        token_index: Self::TokenIndex,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta;

    fn token_maker_delta_mut(
        token_index: Self::TokenIndex,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta>;
}
